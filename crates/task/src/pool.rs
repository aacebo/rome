use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

use crate::{Task, Worker};

pub struct TaskPool {
    name: String,
    capacity: usize,
    next: AtomicUsize,
    workers: Mutex<Vec<Worker>>,
}

impl TaskPool {
    pub fn new(name: impl Into<String>, capacity: usize) -> Self {
        TaskPool {
            name: name.into(),
            capacity,
            next: AtomicUsize::new(0),
            workers: Mutex::new(vec![]),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn start(&self) {
        let mut workers = vec![];

        for _ in 0..self.capacity {
            let worker = Worker::new(self.name.clone());
            worker.start();
            workers.push(worker);
        }

        *self.workers.lock().unwrap() = workers;
    }

    pub fn stop(&self) {
        let mut workers = self.workers.lock().unwrap();

        for worker in workers.drain(..) {
            worker.stop();
        }
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let index = self.next.fetch_add(1, Ordering::Acquire);
        let workers = self.workers.lock().unwrap();

        if index >= workers.len() - 1 {
            self.next.store(0, Ordering::Release);
        }

        workers.get(index).unwrap().spawn(future)
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        self.stop();
    }
}
