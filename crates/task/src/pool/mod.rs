mod command;

pub use command::*;

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::Wake,
};

use crate::{
    Task, internal,
    metrics::{PoolMetrics, PoolMetricsSnapshot},
};

pub struct TaskPool {
    name: String,
    next_id: AtomicU64,
    capacity: usize,
    stopped: AtomicBool,
    metrics: Arc<PoolMetrics>,
    workers: Mutex<Vec<Arc<internal::Worker>>>,
    commands: internal::Channel<Command>,
}

impl TaskPool {
    pub fn new(name: impl Into<String>, capacity: usize) -> Self {
        TaskPool {
            name: name.into(),
            next_id: AtomicU64::new(0),
            capacity,
            stopped: AtomicBool::new(false),
            metrics: Arc::new(PoolMetrics::default()),
            workers: Mutex::new(vec![]),
            commands: internal::Channel::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn metrics(&self) -> PoolMetricsSnapshot {
        self.metrics.snapshot()
    }

    pub fn start(&self) {
        let mut workers = self.workers.lock().unwrap();

        if !workers.is_empty() {
            return;
        }

        for _ in 0..self.capacity {
            let worker = Arc::new(internal::Worker::new());
            worker.start(
                &self.name,
                self.metrics.clone(),
                self.commands.receiver().clone(),
            );
            workers.push(worker);
        }
    }

    pub fn stop(&self) {
        if self.stopped.swap(true, Ordering::AcqRel) {
            return;
        }

        let mut workers = self.workers.lock().unwrap();

        for _ in 0..workers.len() {
            let _ = self.commands.sender().send(Command::stop());
        }

        for worker in workers.drain(..) {
            worker.stop();
        }
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let run = Arc::new(internal::TaskRun::new(
            self.next_id.fetch_add(1, Ordering::SeqCst).into(),
            self.commands.sender().clone(),
            future,
        ));

        run.wake_by_ref();
        self.metrics.tasks.queued.increment();
        Task { run }
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        self.stop();
    }
}
