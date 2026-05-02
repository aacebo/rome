mod command;
mod config;

pub use command::*;
pub use config::*;

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
    next_id: AtomicU64,
    config: PoolConfig,
    stopped: AtomicBool,
    metrics: Arc<PoolMetrics>,
    workers: Mutex<Vec<Arc<internal::Worker>>>,
    commands: internal::Channel<Command>,
}

impl TaskPool {
    pub fn new(config: PoolConfig) -> Self {
        Self {
            next_id: AtomicU64::new(0),
            config,
            stopped: AtomicBool::new(false),
            metrics: Arc::new(PoolMetrics::default()),
            workers: Mutex::new(vec![]),
            commands: internal::Channel::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn metrics(&self) -> PoolMetricsSnapshot {
        self.metrics.snapshot()
    }

    pub fn start(&self) {
        let mut workers = self.workers.lock().unwrap();

        if !workers.is_empty() {
            return;
        }

        let worker = Arc::new(internal::Worker::new());

        worker.start(
            self.name(),
            self.metrics.clone(),
            self.commands.receiver().clone(),
        );

        workers.push(worker);
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
        if std::time::Duration::from_nanos(self.metrics.tasks.spawn_latency_ns.get())
            >= self.config.scale_up_latency
        {
            self.spawn_thread();
        }

        let run = Arc::new(internal::TaskRun::new(
            self.next_id.fetch_add(1, Ordering::SeqCst).into(),
            self.commands.sender().clone(),
            future,
        ));

        run.wake_by_ref();
        self.metrics.tasks.queued.increment();
        Task { run }
    }

    fn spawn_thread(&self) {
        let worker = Arc::new(internal::Worker::new());

        worker.start(
            self.name(),
            self.metrics.clone(),
            self.commands.receiver().clone(),
        );

        self.workers.lock().unwrap().push(worker);
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        self.stop();
    }
}
