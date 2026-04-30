use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::Wake,
    time::Duration,
};

use futures::FutureExt;

use crate::{AtomicTaskStatus, Message, Task, TaskState, TaskStatus, Worker};

pub struct TaskPool {
    name: String,
    capacity: usize,
    next_id: AtomicU64,
    workers: Mutex<Vec<Arc<Worker>>>,
    sender: crossbeam::channel::Sender<Message>,
    receiver: crossbeam::channel::Receiver<Message>,
}

impl TaskPool {
    pub fn new(name: impl Into<String>, capacity: usize) -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();

        TaskPool {
            name: name.into(),
            capacity,
            next_id: AtomicU64::new(0),
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
        }

        *self.workers.lock().unwrap() = workers;
    }

    pub fn stop(&self) {
        let mut workers = self.workers.lock().unwrap();

        for worker in workers.drain(..) {
            let _ = self
                .sender
                .send_timeout(Message::Stop, Duration::from_millis(200))
                .unwrap();

            worker.stop();
        }
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
