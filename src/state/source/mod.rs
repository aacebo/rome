mod operator;
mod stream;

pub use operator::*;
pub use stream::*;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, atomic},
};

use crate::state::Signal;

pub struct Source<T> {
    inner: Arc<_Source<T>>,
}

impl<T> Clone for Source<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Source<T> {
    pub fn new(value: impl Into<Arc<T>>) -> Self {
        Self {
            inner: Arc::new(_Source::new(value)),
        }
    }

    pub fn value(&self) -> Arc<T> {
        self.inner.value()
    }

    pub fn stream(&self) -> Stream<T> {
        let (id, handle) = self.inner.create();
        let source = Arc::downgrade(&self.inner);
        Stream::new(id, handle, source)
    }

    pub fn pipe<O>(&self, op: O) -> O::Output
    where
        O: Operator<Source<T>>,
    {
        op.apply(self.clone())
    }

    pub fn emit(&self, value: impl Into<Arc<T>>) -> &Self {
        let ptr = value.into();
        *self.inner.value.write().unwrap() = ptr.clone();

        for stream in self.inner.snapshot() {
            stream.next(ptr.clone());
        }

        self
    }
}

impl<T: Into<Arc<T>>> From<T> for Source<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Signal for Source<T> {
    type Value = T;

    fn get(&self) -> std::sync::Arc<Self::Value> {
        self.inner.value()
    }

    fn consume(&self) -> impl super::Consumer<Value = Self::Value> {
        let (id, handle) = self.inner.create();
        let source = Arc::downgrade(&self.inner);
        Stream::new(id, handle, source)
    }
}

struct _Source<T> {
    next_id: atomic::AtomicU64,
    value: RwLock<Arc<T>>,
    pool: RwLock<HashMap<u64, Arc<StreamRef<T>>>>,
}

impl<T> _Source<T> {
    fn new(value: impl Into<Arc<T>>) -> Self {
        Self {
            next_id: atomic::AtomicU64::new(1),
            value: RwLock::new(value.into()),
            pool: RwLock::new(HashMap::new()),
        }
    }

    fn value(&self) -> Arc<T> {
        self.value.read().unwrap().clone()
    }

    fn create(&self) -> (u64, Arc<StreamRef<T>>) {
        let id = self.next_id.fetch_add(1, atomic::Ordering::Relaxed);
        let handle = Arc::new(StreamRef::new());
        self.pool.write().unwrap().insert(id, handle.clone());
        (id, handle)
    }

    fn remove(&self, id: u64) {
        self.pool.write().unwrap().remove(&id);
    }

    fn snapshot(&self) -> Vec<Arc<StreamRef<T>>> {
        self.pool.read().unwrap().values().cloned().collect()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.pool.read().unwrap().len()
    }
}

impl<T> Drop for _Source<T> {
    fn drop(&mut self) {
        // Last Arc<_Source> is going away; wake all subscribers so they
        // observe Weak::upgrade() == None on next poll.
        for stream in self.pool.get_mut().unwrap().values() {
            stream.wake();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    fn poll_once<S: futures::Stream + Unpin>(s: &mut S) -> Poll<Option<S::Item>> {
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        Pin::new(s).poll_next(&mut cx)
    }

    #[test]
    fn subscriber_receives_emitted_value() {
        let source = Source::new(0u32);
        let mut stream = source.stream();

        assert!(matches!(poll_once(&mut stream), Poll::Pending));

        source.emit(42);

        match poll_once(&mut stream) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 42),
            other => panic!("expected Ready(Some(42)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn drop_deregisters_subscriber() {
        let source = Source::new(0u32);
        let s1 = source.stream();
        let s2 = source.stream();
        let s3 = source.stream();

        assert_eq!(source.inner.len(), 3);

        drop(s1);
        drop(s2);
        drop(s3);

        assert_eq!(source.inner.len(), 0);
    }

    #[test]
    fn source_drop_terminates_streams() {
        let source = Source::new(0u32);
        let mut stream = source.stream();

        drop(source);

        match poll_once(&mut stream) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn source_drop_with_pending_yields_value_then_none() {
        let source = Source::new(0u32);
        let mut stream = source.stream();

        source.emit(7);
        drop(source);

        match poll_once(&mut stream) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 7),
            other => panic!("expected Ready(Some(7)), got {:?}", other.map(|_| ())),
        }

        match poll_once(&mut stream) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn coalescing_drops_intermediate_values() {
        let source = Source::new(0u32);
        let mut stream = source.stream();

        for i in 1..=100 {
            source.emit(i);
        }

        match poll_once(&mut stream) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 100),
            other => panic!("expected Ready(Some(100)), got {:?}", other.map(|_| ())),
        }

        assert!(matches!(poll_once(&mut stream), Poll::Pending));
    }

    #[test]
    fn stream_is_send_static() {
        fn assert_send_static<T: Send + 'static>() {}
        assert_send_static::<Stream<u32>>();
        assert_send_static::<Source<u32>>();
    }

    #[test]
    fn value_returns_latest() {
        let source = Source::new(1u32);
        assert_eq!(*source.value(), 1);
        source.emit(2);
        assert_eq!(*source.value(), 2);
        source.emit(3);
        assert_eq!(*source.value(), 3);
    }

    #[test]
    fn multiple_subscribers_each_receive() {
        let source = Source::new(0u32);
        let mut s1 = source.stream();
        let mut s2 = source.stream();

        source.emit(99);

        for s in [&mut s1, &mut s2] {
            match poll_once(s) {
                Poll::Ready(Some(v)) => assert_eq!(*v, 99),
                other => panic!("expected Ready(Some(99)), got {:?}", other.map(|_| ())),
            }
        }
    }

    #[test]
    fn dropping_non_last_clone_does_not_terminate_stream() {
        let source = Source::new(0u32);
        let clone = source.clone();
        let mut stream = source.stream();

        drop(clone);

        // The original clone still exists, so the stream must remain open.
        assert!(matches!(poll_once(&mut stream), Poll::Pending));

        source.emit(5);
        match poll_once(&mut stream) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 5),
            other => panic!("expected Ready(Some(5)), got {:?}", other.map(|_| ())),
        }

        // Now drop the last remaining clone; stream terminates.
        drop(source);
        match poll_once(&mut stream) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn stream_is_spawnable_across_tasks() {
        let source = Arc::new(Source::new(0u32));
        let stream = source.stream();

        let task = tokio::spawn(async move {
            let mut stream = stream;
            let mut last = None;
            while let Some(v) = stream.next().await {
                last = Some(*v);
            }
            last
        });

        // Yield so the spawned task can park on poll_next before we emit.
        tokio::task::yield_now().await;
        source.emit(1);
        source.emit(2);
        source.emit(3);

        // Drop the only Source handle: the stream should terminate.
        drop(Arc::try_unwrap(source).ok().expect("only one Arc"));

        let last = task.await.expect("task joined");
        assert!(matches!(last, Some(1) | Some(2) | Some(3)));
    }

    #[test]
    fn pipe_from_source_directly() {
        let source = Source::new(0u32);
        let mapped = source.pipe(map(|v: Arc<u32>| *v + 1));
        let mut sub = mapped.stream();

        assert!(matches!(poll_once(&mut sub), Poll::Pending));

        source.emit(41);
        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 42),
            other => panic!("expected Ready(Some(42)), got {:?}", other.map(|_| ())),
        }
    }
}
