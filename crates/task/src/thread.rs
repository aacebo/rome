use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::Job;

pub enum Message {
    Stop,
    Job(Arc<dyn Job>),
}

pub struct Worker {
    stopping: AtomicBool,
    handle: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            stopping: AtomicBool::new(false),
            handle: Mutex::new(None),
        }
    }

    pub fn start(&self, pool: impl Into<String>, receiver: crossbeam::channel::Receiver<Message>) {
        let pool = pool.into();
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

        *self.handle.lock().unwrap() = Some(handle);
    }

    pub fn stop(&self) {
        if self.stopping.swap(true, Ordering::AcqRel) {
            tracing::trace!(target: "ayr::task::thread", "stop already in progress");
            return;
        }

        let _ = self.handle.lock().unwrap().take().map(|v| v.join());
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if !self.stopping.load(Ordering::Acquire) {
            self.stop();
        }
    }
}
