use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
    task::Wake,
    time::Duration,
};

use futures::FutureExt;

use crate::{AtomicTaskStatus, Message, Task, TaskState, TaskStatus, Worker};

pub struct TaskPool {
    name: String,
    next_id: AtomicU64,
    size: AtomicUsize,
    capacity: usize,
    workers: Mutex<Vec<Arc<Worker>>>,
    sender: crossbeam::channel::Sender<Message>,
    receiver: crossbeam::channel::Receiver<Message>,
}

impl TaskPool {
    pub fn new(name: impl Into<String>, capacity: usize) -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();

        TaskPool {
            name: name.into(),
            next_id: AtomicU64::new(0),
            size: AtomicUsize::new(0),
            capacity,
            workers: Mutex::new(vec![]),
            sender,
            receiver,
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
            let worker = Arc::new(Worker::new());
            worker.start(&self.name, self.receiver.clone());
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
                .sender
                .send_timeout(Message::Stop, Duration::from_millis(200))
                .unwrap();
        }

        let _ = workers.drain(..).map(|w| w.stop());
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let state = Arc::new(TaskState {
            id: self.next_id.fetch_add(1, Ordering::SeqCst).into(),
            status: AtomicTaskStatus::new(TaskStatus::default()),
            aborted: AtomicBool::new(false),
            join: Mutex::new(None),
            sender: self.sender.clone(),
            output: Mutex::new(None),
            future: Mutex::new(Some(future.boxed())),
        });

        state.wake_by_ref();
        Task { state }
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        self.stop();
    }
}
