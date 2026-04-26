mod read;
mod select;

pub use read::*;
pub use select::*;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, atomic},
};

pub struct Signal<T> {
    inner: Arc<_Signal<T>>,
}

impl<T> Signal<T> {
    pub fn new(value: impl Into<Arc<T>>) -> Self {
        Self {
            inner: Arc::new(_Signal::new(value)),
        }
    }

    pub fn get(&self) -> Arc<T> {
        self.inner.get()
    }

    pub fn stream(&self) -> Reader<T> {
        let (id, handle) = self.inner.create();
        let signal = Arc::downgrade(&self.inner);
        Reader::new(id, handle, signal)
    }

    pub fn emit(&self, value: impl Into<Arc<T>>) -> &Self {
        let ptr = value.into();
        *self.inner.value.write().unwrap() = ptr.clone();

        for reader in self.inner.snapshot() {
            reader.next(ptr.clone());
        }

        self
    }
}

impl<T: Clone> Signal<T> {
    /// Mutate the inner value in place using `Arc::make_mut` semantics:
    /// if no other `Arc<T>` is outstanding, mutate without cloning;
    /// otherwise clone, mutate the clone, swap in the new `Arc`. After
    /// `f` returns, every subscriber is notified with the resulting `Arc<T>`.
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.inner.value.write().unwrap();
        let result = f(Arc::make_mut(&mut *guard));
        let snapshot = guard.clone();
        drop(guard);

        for reader in self.inner.snapshot() {
            reader.next(snapshot.clone());
        }

        result
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Into<Arc<T>>> From<T> for Signal<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T, U: ?Sized> PartialEq<U> for Signal<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &U) -> bool {
        self.get().as_ref() == other
    }
}

impl<T, U: ?Sized> PartialOrd<U> for Signal<T>
where
    T: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &U) -> Option<std::cmp::Ordering> {
        self.get().as_ref().partial_cmp(other)
    }
}

struct _Signal<T> {
    next_id: atomic::AtomicU64,
    value: RwLock<Arc<T>>,
    pool: RwLock<HashMap<u64, Arc<ReaderRef<T>>>>,
}

impl<T> _Signal<T> {
    fn new(value: impl Into<Arc<T>>) -> Self {
        Self {
            next_id: atomic::AtomicU64::new(1),
            value: RwLock::new(value.into()),
            pool: RwLock::new(HashMap::new()),
        }
    }

    fn get(&self) -> Arc<T> {
        self.value.read().unwrap().clone()
    }

    fn create(&self) -> (u64, Arc<ReaderRef<T>>) {
        let id = self.next_id.fetch_add(1, atomic::Ordering::Relaxed);
        let handle = Arc::new(ReaderRef::new());
        handle.next(self.value.read().unwrap().clone());
        self.pool.write().unwrap().insert(id, handle.clone());
        (id, handle)
    }

    fn remove(&self, id: u64) {
        self.pool.write().unwrap().remove(&id);
    }

    fn snapshot(&self) -> Vec<Arc<ReaderRef<T>>> {
        self.pool.read().unwrap().values().cloned().collect()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.pool.read().unwrap().len()
    }
}

impl<T> Drop for _Signal<T> {
    fn drop(&mut self) {
        // Last Arc<_Signal> is going away; wake all subscribers so they
        // observe Weak::upgrade() == None on next poll.
        for reader in self.pool.get_mut().unwrap().values() {
            reader.wake();
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
        let signal = Signal::new(0u32);
        let mut reader = signal.stream();

        // Fresh stream is seeded with the current value.
        match poll_once(&mut reader) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 0),
            other => panic!("expected Ready(Some(0)), got {:?}", other.map(|_| ())),
        }
        assert!(matches!(poll_once(&mut reader), Poll::Pending));

        signal.emit(42);

        match poll_once(&mut reader) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 42),
            other => panic!("expected Ready(Some(42)), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn drop_deregisters_subscriber() {
        let signal = Signal::new(0u32);
        let s1 = signal.stream();
        let s2 = signal.stream();
        let s3 = signal.stream();

        assert_eq!(signal.inner.len(), 3);

        drop(s1);
        drop(s2);
        drop(s3);

        assert_eq!(signal.inner.len(), 0);
    }

    #[test]
    fn signal_drop_terminates_readers() {
        let signal = Signal::new(0u32);
        let mut reader = signal.stream();

        drop(signal);

        // Seeded current value still drains before termination.
        match poll_once(&mut reader) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 0),
            other => panic!("expected Ready(Some(0)), got {:?}", other.map(|_| ())),
        }

        match poll_once(&mut reader) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn signal_drop_with_pending_yields_value_then_none() {
        let signal = Signal::new(0u32);
        let mut reader = signal.stream();

        signal.emit(7);
        drop(signal);

        match poll_once(&mut reader) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 7),
            other => panic!("expected Ready(Some(7)), got {:?}", other.map(|_| ())),
        }

        match poll_once(&mut reader) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[test]
    fn coalescing_drops_intermediate_values() {
        let signal = Signal::new(0u32);
        let mut reader = signal.stream();

        for i in 1..=100 {
            signal.emit(i);
        }

        match poll_once(&mut reader) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 100),
            other => panic!("expected Ready(Some(100)), got {:?}", other.map(|_| ())),
        }

        assert!(matches!(poll_once(&mut reader), Poll::Pending));
    }

    #[test]
    fn reader_is_send_static() {
        fn assert_send_static<T: Send + 'static>() {}
        assert_send_static::<Reader<u32>>();
        assert_send_static::<Signal<u32>>();
    }

    #[test]
    fn value_returns_latest() {
        let signal = Signal::new(1u32);
        assert_eq!(*signal.get(), 1);
        signal.emit(2);
        assert_eq!(*signal.get(), 2);
        signal.emit(3);
        assert_eq!(*signal.get(), 3);
    }

    #[test]
    fn multiple_subscribers_each_receive() {
        let signal = Signal::new(0u32);
        let mut s1 = signal.stream();
        let mut s2 = signal.stream();

        signal.emit(99);

        for s in [&mut s1, &mut s2] {
            match poll_once(s) {
                Poll::Ready(Some(v)) => assert_eq!(*v, 99),
                other => panic!("expected Ready(Some(99)), got {:?}", other.map(|_| ())),
            }
        }
    }

    #[test]
    fn dropping_non_last_clone_does_not_terminate_reader() {
        let signal = Signal::new(0u32);
        let clone = signal.clone();
        let mut reader = signal.stream();

        drop(clone);

        // The original clone still exists, so the reader must remain open.
        // Fresh stream is seeded with the current value (0), drain it first.
        match poll_once(&mut reader) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 0),
            other => panic!("expected Ready(Some(0)), got {:?}", other.map(|_| ())),
        }
        assert!(matches!(poll_once(&mut reader), Poll::Pending));

        signal.emit(5);
        match poll_once(&mut reader) {
            Poll::Ready(Some(v)) => assert_eq!(*v, 5),
            other => panic!("expected Ready(Some(5)), got {:?}", other.map(|_| ())),
        }

        // Now drop the last remaining clone; reader terminates.
        drop(signal);
        match poll_once(&mut reader) {
            Poll::Ready(None) => {}
            other => panic!("expected Ready(None), got {:?}", other.map(|_| ())),
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn reader_is_spawnable_across_tasks() {
        let signal = Arc::new(Signal::new(0u32));
        let reader = signal.stream();

        let task = tokio::spawn(async move {
            let mut reader = reader;
            let mut last = None;
            while let Some(v) = reader.next().await {
                last = Some(*v);
            }
            last
        });

        // Yield so the spawned task can park on poll_next before we emit.
        tokio::task::yield_now().await;
        signal.emit(1);
        signal.emit(2);
        signal.emit(3);

        // Drop the only Signal handle: the reader should terminate.
        drop(Arc::try_unwrap(signal).ok().expect("only one Arc"));

        let last = task.await.expect("task joined");
        assert!(matches!(last, Some(1) | Some(2) | Some(3)));
    }

    #[test]
    fn map_via_stream_ext() {
        let signal = Signal::new(0u32);
        let mut sub = signal.stream().map(|v: Arc<u32>| *v + 1);

        // Fresh stream is seeded with the current value (0); mapped to 1.
        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(v, 1),
            other => panic!("expected Ready(Some(1)), got {:?}", other.map(|_| ())),
        }

        signal.emit(41);
        match poll_once(&mut sub) {
            Poll::Ready(Some(v)) => assert_eq!(v, 42),
            other => panic!("expected Ready(Some(42)), got {:?}", other.map(|_| ())),
        }
    }
}
