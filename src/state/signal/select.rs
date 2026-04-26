use std::sync::Arc;

use futures::{Stream, StreamExt};

use super::Signal;

pub struct Select<TIn, TOut> {
    upstream: Signal<TIn>,
    project: Arc<dyn Fn(&TIn) -> TOut + Send + Sync + 'static>,
}

impl<TIn, TOut> Select<TIn, TOut>
where
    TIn: Send + Sync + 'static,
    TOut: Send + 'static,
{
    pub fn new(
        upstream: Signal<TIn>,
        project: impl Fn(&TIn) -> TOut + Send + Sync + 'static,
    ) -> Self {
        Self {
            upstream,
            project: Arc::new(project),
        }
    }

    pub fn get(&self) -> TOut {
        (self.project)(&self.upstream.get())
    }

    pub fn stream(&self) -> impl Stream<Item = TOut> + Send + 'static {
        let project = self.project.clone();
        self.upstream.stream().map(move |arc_in| project(&arc_in))
    }
}

impl<TIn, TOut> Clone for Select<TIn, TOut> {
    fn clone(&self) -> Self {
        Self {
            upstream: self.upstream.clone(),
            project: self.project.clone(),
        }
    }
}

impl<TIn, TOut> std::fmt::Debug for Select<TIn, TOut>
where
    TIn: Send + Sync + 'static,
    TOut: std::fmt::Debug + Send + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<TIn, TOut, U: ?Sized> PartialEq<U> for Select<TIn, TOut>
where
    TIn: Send + Sync + 'static,
    TOut: Send + 'static + PartialEq<U>,
{
    fn eq(&self, other: &U) -> bool {
        self.get() == *other
    }
}

impl<TIn, TOut, U: ?Sized> PartialOrd<U> for Select<TIn, TOut>
where
    TIn: Send + Sync + 'static,
    TOut: Send + 'static + PartialOrd<U>,
{
    fn partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    fn poll_once<S: Stream + Unpin>(s: &mut S) -> Poll<Option<S::Item>> {
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        Pin::new(s).poll_next(&mut cx)
    }

    #[test]
    fn get_returns_projected_current() {
        let signal = Signal::new(3u32);
        let v = Select::new(signal.clone(), |v: &u32| *v * 2);
        assert_eq!(v.get(), 6);
    }

    #[test]
    fn stream_first_yield_is_projected_initial() {
        let signal = Signal::new(5u32);
        let v = Select::new(signal.clone(), |v: &u32| *v + 1);
        let mut sub = Box::pin(v.stream());
        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(v, 6),
            other => panic!("expected Ready(Some(6)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn subsequent_emits_propagate() {
        let signal = Signal::new(0u32);
        let v = Select::new(signal.clone(), |v: &u32| *v * 10);
        let mut sub = Box::pin(v.stream());
        let _ = poll_once(&mut sub); // drain seeded value

        signal.emit(7);
        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(v, 70),
            other => panic!("expected Ready(Some(70)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn dropping_all_signal_handles_terminates_stream() {
        let signal = Signal::new(0u32);
        let v = Select::new(signal.clone(), |v: &u32| *v);
        let mut sub = Box::pin(v.stream());
        let _ = poll_once(&mut sub); // drain seeded value

        // Select holds its own clone of upstream, so we must drop it too.
        drop(v);
        drop(signal);
        match poll_once(&mut sub) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn is_send_static() {
        fn assert_send_static<T: Send + 'static>() {}
        assert_send_static::<Select<u32, u32>>();
        assert_send_static::<Select<u32, String>>();
    }
}
