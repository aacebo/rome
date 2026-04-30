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

use crate::{AtomicTaskStatus, Job, Task, TaskState, TaskStatus};

pub(crate) enum Message {
    Stop,
    Job(Arc<dyn Job>),
}

pub struct Worker {
    pool: String,
    thread_id: Mutex<Option<ThreadId>>,
    next_id: AtomicU64,
    stopping: AtomicBool,
    sender: crossbeam::channel::Sender<Message>,
    receiver: crossbeam::channel::Receiver<Message>,
    handle: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl Worker {
    pub fn new(pool: impl Into<String>) -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();

        Self {
            pool: pool.into(),
            thread_id: Mutex::new(None),
            next_id: AtomicU64::new(0),
            stopping: AtomicBool::new(false),
            sender,
            receiver,
            handle: Mutex::new(None),
        }
    }

    pub fn start(&self) {
        let pool = self.pool.clone();
        let receiver = self.receiver.clone();
        let handle = std::thread::Builder::new()
            .name(format!("task::pool::{}::thread", &pool,))
            .spawn(move || {
                let thread_id = format!("{:?}", std::thread::current().id())
                    .replace("ThreadId(", "")
                    .replace(")", "")
                    .trim()
                    .to_string();

                let span = tracing::debug_span!(target: "ayr::task::thread", "worker", thread_id = %thread_id);
                let _enter = span.enter();
                tracing::debug!(target: "ayr::task::thread", "starting");

                loop {
                    match receiver.try_recv() {
                        Err(crossbeam::channel::TryRecvError::Disconnected) => {
                            tracing::debug!(target: "ayr::task::thread", "disconnected");
                            break;
                        }
                        Err(crossbeam::channel::TryRecvError::Empty) => {
                            std::thread::sleep(Duration::from_millis(200));
                        }
                        Ok(Message::Stop) => {
                            tracing::debug!(target: "ayr::task::thread", "stopping");
                            break;
                        }
                        Ok(Message::Job(job)) => {
                            tracing::trace!(target: "ayr::task::thread", task_id = %job.task_id(), "running");
                            job.run();
                        }
                    }
                }

                tracing::debug!(target: "ayr::task::thread", "exiting");
            })
            .expect("failed to start task worker thread");

        *self.thread_id.lock().unwrap() = Some(handle.thread().id());
        *self.handle.lock().unwrap() = Some(handle);
    }

    pub fn stop(&self) {
        if self.stopping.swap(true, Ordering::AcqRel) {
            tracing::trace!(target: "ayr::task::thread", "stop already in progress");
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
        if !self.stopping.load(Ordering::Acquire) {
            self.stop();
        }
    }
}
