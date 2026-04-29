use std::{
    sync::{Arc, Mutex, atomic::AtomicBool},
    task::Wake,
};

use futures::FutureExt;

use crate::{AtomicTaskState, Run, Task, TaskCell, TaskState};

pub struct Executor {
    sender: crossbeam::channel::Sender<Arc<dyn Run>>,
    receiver: crossbeam::channel::Receiver<Arc<dyn Run>>,
}

impl Executor {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();
        Self { sender, receiver }
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let cell = Arc::new(TaskCell {
            state: AtomicTaskState::new(TaskState::default()),
            aborted: AtomicBool::new(false),
            join: Mutex::new(None),
            sender: self.sender.clone(),
            output: Mutex::new(None),
            future: Mutex::new(Some(future.boxed())),
        });

        cell.wake_by_ref();

        Task {
            cell,
            receiver: self.receiver.clone(),
        }
    }
}
