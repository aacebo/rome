use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use super::Operator;
use crate::state::signal::{Reader, Signal};

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
    upreader: Mutex<Option<Reader<In>>>,
    callback: Mutex<Box<dyn FnMut(Arc<In>) -> Out + Send>>,
    inner: Signal<Out>,
    done: AtomicBool,
}

impl<In, Out> Mapped<In, Out>
where
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
{
    pub fn reader(&self) -> MappedReader<In, Out> {
        MappedReader {
            subscriber: self.state.inner.reader(),
            state: self.state.clone(),
        }
    }
}

impl<In, Out> Deref for Mapped<In, Out> {
    type Target = Signal<Out>;

    fn deref(&self) -> &Signal<Out> {
        &self.state.inner
    }
}

pub struct MappedReader<In, Out> {
    state: Arc<MappedState<In, Out>>,
    subscriber: Reader<Out>,
}

impl<In, Out> futures::Stream for MappedReader<In, Out>
where
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
{
    type Item = Arc<Out>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        #[allow(clippy::collapsible_if)]
        if let Ok(mut up_slot) = this.state.upreader.try_lock() {
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

impl<In, F, Out> Operator<Signal<In>> for Map<F>
where
    F: FnMut(Arc<In>) -> Out + Send + 'static,
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
{
    type Output = Mapped<In, Out>;

    fn apply(mut self, signal: Signal<In>) -> Mapped<In, Out> {
        let initial = (self.callback)(signal.value());
        let upreader = signal.reader();

        Mapped {
            state: Arc::new(MappedState {
                upreader: Mutex::new(Some(upreader)),
                callback: Mutex::new(Box::new(self.callback)),
                inner: Signal::new(initial),
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
    use crate::state::signal::Signal;

    fn poll_once<S: futures::Stream + Unpin>(s: &mut S) -> Poll<Option<S::Item>> {
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        Pin::new(s).poll_next(&mut cx)
    }

    #[test]
    fn transforms_values() {
        let signal = Signal::new(1u32);
        let mapped = Map {
            callback: |v: Arc<u32>| *v * 2,
        }
        .apply(signal.clone());
        let mut sub = mapped.reader();

        assert!(matches!(poll_once(&mut sub), Poll::Pending));

        signal.emit(5);
        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 10),
            other => panic!("expected Ready(Some(10)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn value_returns_mapped_initial() {
        let signal = Signal::new(3u32);
        let mapped = Map {
            callback: |v: Arc<u32>| *v * 2,
        }
        .apply(signal);
        assert_eq!(*mapped.value(), 6);
    }

    #[test]
    fn chains() {
        let signal = Signal::new(0u32);
        let m1 = Map {
            callback: |v: Arc<u32>| *v + 1,
        }
        .apply(signal.clone());
        let m2 = Map {
            callback: |v: Arc<u32>| *v * 10,
        }
        .apply((*m1).clone());
        let mut sub = m2.reader();
        let mut m1_driver = m1.reader();

        assert!(matches!(poll_once(&mut sub), Poll::Pending));

        signal.emit(4);
        // Drive m1's pump so its inner Signal emits 5; m2's pump (via sub) then sees it.
        let _ = poll_once(&mut m1_driver);

        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 50),
            other => panic!("expected Ready(Some(50)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn terminates_on_signal_drop() {
        let signal = Signal::new(0u32);
        let mapped = Map {
            callback: |v: Arc<u32>| *v,
        }
        .apply(signal.clone());
        let mut sub = mapped.reader();

        drop(signal);

        match poll_once(&mut sub) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn is_send_static() {
        fn assert_send_static<T: Send + 'static>() {}
        assert_send_static::<Mapped<u32, u32>>();
        assert_send_static::<MappedReader<u32, u32>>();
    }
}
