use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{Task, Worker};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PoolId(u64);

impl PoolId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl From<u64> for PoolId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for PoolId {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

pub struct TaskPool {
    id: PoolId,
    next: AtomicUsize,
    workers: Vec<Worker>,
}

impl TaskPool {
    pub fn sizeof(id: PoolId, size: usize) -> Self {
        let mut workers = vec![];

        for _ in 0..size {
            workers.push(Worker::new(id));
        }

        TaskPool {
            id,
            next: AtomicUsize::new(0),
            workers,
        }
    }

    pub fn id(&self) -> PoolId {
        self.id
    }

    pub fn start(&self) {
        for (i, worker) in self.workers.iter().enumerate() {
            worker.start(i);
        }
    }

    pub fn stop(&self) {
        for worker in &self.workers {
            worker.stop();
        }
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let index = self.next.fetch_add(1, Ordering::Acquire);

        if index >= self.workers.len() - 1 {
            self.next.store(0, Ordering::Release);
        }

        self.workers.get(index).unwrap().spawn(future)
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        self.stop();
    }
}
