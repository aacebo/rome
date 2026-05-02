use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use crate::{Command, TaskStatus, metrics::PoolMetrics};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ThreadStatus {
    Idle,
    Active,
}

pub(crate) struct Worker {
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

    pub fn start(
        &self,
        pool: impl Into<String>,
        metrics: Arc<PoolMetrics>,
        commands: crossbeam::channel::Receiver<Command>,
    ) {
        let pool = pool.into();
        let handle = std::thread::Builder::new()
            .name(format!("task::pool::{}::thread", &pool,))
            .spawn(move || {
                let thread_id = format!("{:?}", std::thread::current().id())
                    .replace("ThreadId(", "")
                    .replace(")", "")
                    .trim()
                    .to_string();

                let mut thread_status = ThreadStatus::Idle;
                let span = tracing::debug_span!(target: "ayr::task::thread", "worker", thread_id = %thread_id);
                let _enter = span.enter();

                tracing::debug!(target: "ayr::task::thread", "starting");
                metrics.threads.spawned.increment();

                loop {
                    let status = match commands.recv_timeout(std::time::Duration::from_millis(200)) {
                        Err(crossbeam::channel::RecvTimeoutError::Timeout) => {
                            if thread_status == ThreadStatus::Active {
                                thread_status = ThreadStatus::Idle;
                                metrics.threads.idle.increment();
                            }

                            continue;
                        },
                        Ok(Command::Stop(_)) | Err(_) => break,
                        Ok(Command::Spawn(timestamp, job)) => {
                            metrics.tasks.spawned.increment();
                            metrics
                                .tasks
                                .spawn_latency_ns
                                .add((std::time::Instant::now() - timestamp).as_nanos() as u64);
                            job.run()
                        },
                        Ok(Command::Tick(timestamp, job)) => {
                            metrics
                                .tasks
                                .spawn_latency_ns
                                .add((std::time::Instant::now() - timestamp).as_nanos() as u64);
                            job.run()
                        }
                    };

                    if thread_status == ThreadStatus::Idle {
                        thread_status = ThreadStatus::Active;
                        metrics.threads.active.increment();
                    }

                    if status == TaskStatus::Complete {
                        metrics.tasks.completed.increment();
                    }
                }

                tracing::debug!(target: "ayr::task::thread", "exiting");
                metrics.threads.dropped.increment();
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
