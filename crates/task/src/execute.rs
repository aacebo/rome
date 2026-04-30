use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

use crate::{PoolId, Task, TaskPool};

pub struct Executor {
    next: AtomicUsize,
    size: usize,
    pools: Mutex<Vec<TaskPool>>,
}

impl Executor {
    pub fn new() -> Self {
        Self::sizeof(1)
    }

    pub fn sizeof(size: usize) -> Self {
        let mut pools = vec![];
        let max = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        assert!(size <= max);
        let pool_size = max / size;

        for i in 0..size {
            pools.push(TaskPool::sizeof(PoolId::from(i), pool_size));
        }

        Self {
            next: AtomicUsize::new(0),
            size,
            pools: Mutex::new(pools),
        }
    }

    pub fn start(&self) {
        let pools = self.pools.lock().unwrap();

        for pool in pools.iter() {
            pool.start();
        }
    }

    pub fn stop(&self) {
        let pools = self.pools.lock().unwrap();

        for pool in pools.iter() {
            pool.stop();
        }
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let index = self.next.fetch_add(1, Ordering::SeqCst);

        if index >= self.size - 1 {
            self.next.store(0, Ordering::Release);
        }

        self.pools.lock().unwrap().get(index).unwrap().spawn(future)
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        self.stop();
    }
}
