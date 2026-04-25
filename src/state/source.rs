use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard, atomic},
};

pub struct Source<T> {
    value: RwLock<Arc<T>>,
    pool: Arc<Pool<T>>,
}

impl<T> Source<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: RwLock::new(Arc::new(value)),
            pool: Arc::new(Pool::new()),
        }
    }

    pub fn value(&self) -> Arc<T> {
        self.value.read().unwrap().clone()
    }

    pub fn stream(&self) -> Stream<T> {
        let (id, handle) = self.pool.create();
        let pool = self.pool.clone();
        Stream { id, handle, pool }
    }

    pub fn emit(&self, value: T) -> &Self {
        let ptr = Arc::new(value);
        *self.value.write().unwrap() = ptr.clone();
        let pool = self.pool.read().values().cloned().collect::<Vec<_>>();

        for stream in pool {
            stream.next(ptr.clone());
        }

        self
    }
}

impl<T> From<T> for Source<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

pub struct Stream<T> {
    id: u64,
    handle: Arc<StreamRef<T>>,
    pool: Arc<Pool<T>>,
}

impl<T> Drop for Stream<T> {
    fn drop(&mut self) {
        self.pool.write().remove(&self.id);
    }
}

impl<T> futures::Stream for Stream<T> {
    type Item = Arc<T>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if let Some(v) = self.handle.pending.lock().unwrap().take() {
            return std::task::Poll::Ready(Some(v));
        }

        self.handle.waker.register(cx.waker());
        std::task::Poll::Pending
    }
}

struct Pool<T> {
    next_id: atomic::AtomicU64,
    streams: RwLock<HashMap<u64, Arc<StreamRef<T>>>>,
}

impl<T> Pool<T> {
    fn new() -> Self {
        Self {
            next_id: atomic::AtomicU64::new(1),
            streams: RwLock::new(HashMap::new()),
        }
    }

    fn read(&self) -> RwLockReadGuard<'_, HashMap<u64, Arc<StreamRef<T>>>> {
        self.streams.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<'_, HashMap<u64, Arc<StreamRef<T>>>> {
        self.streams.write().unwrap()
    }

    fn create(&self) -> (u64, Arc<StreamRef<T>>) {
        let id = self.next_id.fetch_add(1, atomic::Ordering::Relaxed);
        let handle = Arc::new(StreamRef::new());
        self.streams.write().unwrap().insert(id, handle.clone());
        (id, handle)
    }
}

struct StreamRef<T> {
    waker: futures::task::AtomicWaker,
    pending: Mutex<Option<Arc<T>>>,
}

impl<T> StreamRef<T> {
    fn new() -> Self {
        Self {
            waker: futures::task::AtomicWaker::new(),
            pending: Mutex::new(None),
        }
    }

    fn next(&self, value: Arc<T>) {
        *self.pending.lock().unwrap() = Some(value);
        self.waker.wake();
    }
}
