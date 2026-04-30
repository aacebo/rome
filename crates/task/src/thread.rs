use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::Wake,
    thread::ThreadId,
    time::Duration,
};

use futures::FutureExt;

use crate::{AtomicTaskStatus, Job, PoolId, Task, TaskState, TaskStatus};

pub(crate) enum Message {
    Stop,
    Job(Arc<dyn Job>),
}

pub struct Worker {
    pool_id: PoolId,
    thread_id: Mutex<Option<ThreadId>>,
    next_id: AtomicU64,
    stopping: AtomicBool,
    sender: crossbeam::channel::Sender<Message>,
    receiver: crossbeam::channel::Receiver<Message>,
    handle: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl Worker {
    pub fn new(pool_id: PoolId) -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();

        Self {
            pool_id,
            thread_id: Mutex::new(None),
            next_id: AtomicU64::new(0),
            stopping: AtomicBool::new(false),
            sender,
            receiver,
            handle: Mutex::new(None),
        }
    }

    pub fn start(&self, i: usize) {
        let pool_id = self.pool_id;
        let receiver = self.receiver.clone();
        let handle = std::thread::Builder::new()
            .name(format!("task::pool::{}::thread::{}", pool_id.as_usize(), i,))
            .spawn(move || {
                loop {
                    match receiver.try_recv() {
                        Err(crossbeam::channel::TryRecvError::Disconnected) => break,
                        Err(crossbeam::channel::TryRecvError::Empty) => {
                            std::thread::sleep(Duration::from_millis(200))
                        }
                        Ok(message) => match message {
                            Message::Stop => break,
                            Message::Job(job) => job.run(),
                        },
                    }
                }
            })
            .expect("failed to start task worker thread");

        *self.thread_id.lock().unwrap() = Some(handle.thread().id());
        *self.handle.lock().unwrap() = Some(handle);
    }

    pub fn stop(&self) {
        if self.stopping.swap(true, Ordering::AcqRel) {
            return;
        }

        let _ = self
            .sender
            .send_timeout(Message::Stop, Duration::from_millis(200))
            .unwrap();

        let _ = self.handle.lock().unwrap().take().map(|v| v.join());
        let _ = self.thread_id.lock().unwrap().take();
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        let thread_id = self
            .thread_id
            .lock()
            .unwrap()
            .expect("spawn called while worker has no running thread");

        let state = Arc::new(TaskState {
            id: self.next_id.fetch_add(1, Ordering::SeqCst).into(),
            thread_id,
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

impl Drop for Worker {
    fn drop(&mut self) {
        self.stop();
    }
}
