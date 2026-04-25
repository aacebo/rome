use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use super::Operator;
use crate::state::source::{Source, Stream};

pub fn map<Callback>(callback: Callback) -> Map<Callback> {
    Map { callback }
}

pub struct Map<F> {
    callback: F,
}

pub struct Mapped<In, Out> {
    state: Arc<MappedState<In, Out>>,
    _input: PhantomData<fn(In)>,
}

struct MappedState<In, Out> {
    upstream: Mutex<Option<Stream<In>>>,
    callback: Mutex<Box<dyn FnMut(Arc<In>) -> Out + Send>>,
    inner: Source<Out>,
    done: AtomicBool,
}

impl<In, Out> Mapped<In, Out>
where
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
{
    pub fn stream(&self) -> MappedStream<In, Out> {
        MappedStream {
            subscriber: self.state.inner.stream(),
            state: self.state.clone(),
        }
    }
}

impl<In, Out> Deref for Mapped<In, Out> {
    type Target = Source<Out>;

    fn deref(&self) -> &Source<Out> {
        &self.state.inner
    }
}

pub struct MappedStream<In, Out> {
    state: Arc<MappedState<In, Out>>,
    subscriber: Stream<Out>,
}

impl<In, Out> futures::Stream for MappedStream<In, Out>
where
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
{
    type Item = Arc<Out>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        #[allow(clippy::collapsible_if)]
        if let Ok(mut up_slot) = this.state.upstream.try_lock() {
            if let Some(up) = up_slot.as_mut() {
                let mut f = this.state.callback.lock().unwrap();
                loop {
                    match Pin::new(&mut *up).poll_next(cx) {
                        Poll::Ready(Some(v)) => {
                            this.state.inner.emit((f)(v));
                        }
                        Poll::Ready(None) => {
                            *up_slot = None;
                            this.state.done.store(true, Ordering::Release);
                            break;
                        }
                        Poll::Pending => break,
                    }
                }
            }
        }

        match Pin::new(&mut this.subscriber).poll_next(cx) {
            Poll::Ready(v) => Poll::Ready(v),
            Poll::Pending => {
                if this.state.done.load(Ordering::Acquire) {
                    Poll::Ready(None)
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

impl<In, F, Out> Operator<Source<In>> for Map<F>
where
    F: FnMut(Arc<In>) -> Out + Send + 'static,
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
{
    type Output = Mapped<In, Out>;

    fn apply(mut self, source: Source<In>) -> Mapped<In, Out> {
        let initial = (self.callback)(source.value());
        let upstream = source.stream();

        Mapped {
            state: Arc::new(MappedState {
                upstream: Mutex::new(Some(upstream)),
                callback: Mutex::new(Box::new(self.callback)),
                inner: Source::new(initial),
                done: AtomicBool::new(false),
            }),
            _input: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::sync::Arc;
    use std::task::{Context, Poll};

    use super::*;
    use crate::state::source::Source;

    fn poll_once<S: futures::Stream + Unpin>(s: &mut S) -> Poll<Option<S::Item>> {
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        Pin::new(s).poll_next(&mut cx)
    }

    #[test]
    fn transforms_values() {
        let source = Source::new(1u32);
        let mapped = Map {
            callback: |v: Arc<u32>| *v * 2,
        }
        .apply(source.clone());
        let mut sub = mapped.stream();

        assert!(matches!(poll_once(&mut sub), Poll::Pending));

        source.emit(5);
        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 10),
            other => panic!("expected Ready(Some(10)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn value_returns_mapped_initial() {
        let source = Source::new(3u32);
        let mapped = Map {
            callback: |v: Arc<u32>| *v * 2,
        }
        .apply(source);
        assert_eq!(*mapped.value(), 6);
    }

    #[test]
    fn chains() {
        let source = Source::new(0u32);
        let m1 = Map {
            callback: |v: Arc<u32>| *v + 1,
        }
        .apply(source.clone());
        let m2 = Map {
            callback: |v: Arc<u32>| *v * 10,
        }
        .apply((*m1).clone());
        let mut sub = m2.stream();
        let mut m1_driver = m1.stream();

        assert!(matches!(poll_once(&mut sub), Poll::Pending));

        source.emit(4);
        // Drive m1's pump so its inner Source emits 5; m2's pump (via sub) then sees it.
        let _ = poll_once(&mut m1_driver);

        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 50),
            other => panic!("expected Ready(Some(50)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn terminates_on_source_drop() {
        let source = Source::new(0u32);
        let mapped = Map {
            callback: |v: Arc<u32>| *v,
        }
        .apply(source.clone());
        let mut sub = mapped.stream();

        drop(source);

        match poll_once(&mut sub) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn is_send_static() {
        fn assert_send_static<T: Send + 'static>() {}
        assert_send_static::<Mapped<u32, u32>>();
        assert_send_static::<MappedStream<u32, u32>>();
    }
}
