mod command;
mod event;
mod metrics;

pub use command::*;
pub use event::*;
pub use metrics::*;

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
    task::Wake,
    time::Duration,
};

use crate::{Task, internal};

pub struct TaskPool {
    name: String,
    next_id: AtomicU64,
    size: AtomicUsize,
    capacity: usize,
    metrics: Arc<TaskPoolMetrics>,
    workers: Mutex<Vec<Arc<internal::Worker>>>,
    events: internal::Channel<Event>,
    commands: internal::Channel<Command>,
}

impl TaskPool {
    pub fn new(name: impl Into<String>, capacity: usize) -> Self {
        TaskPool {
            name: name.into(),
            next_id: AtomicU64::new(0),
            size: AtomicUsize::new(0),
            capacity,
            metrics: Arc::new(TaskPoolMetrics::default()),
            workers: Mutex::new(vec![]),
            events: internal::Channel::new(),
            commands: internal::Channel::new(),
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
            let worker = Arc::new(internal::Worker::new());

            worker.start(
                &self.name,
                self.metrics.clone(),
                self.commands.receiver().clone(),
            );

            workers.push(worker);
            self.size.fetch_add(1, Ordering::Relaxed);
        }

        *self.workers.lock().unwrap() = workers;
    }

    pub fn stop(&self) {
        let mut workers = self.workers.lock().unwrap();
        let size = self.size.load(Ordering::Acquire);

        for _ in 0..size {
            let _ = self
                .commands
                .sender()
                .send_timeout(Command::Stop, Duration::from_millis(200));
        }

        let _ = workers.drain(..).map(|w| w.stop());
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let run = Arc::new(internal::TaskRun::new(
            self.next_id.fetch_add(1, Ordering::SeqCst).into(),
            self.events.sender().clone(),
            self.commands.sender().clone(),
            future,
        ));

        run.wake_by_ref();
        Task { run }
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        self.stop();
    }
}
