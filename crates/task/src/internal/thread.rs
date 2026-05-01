use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use crate::{Command, TaskPoolMetrics, TaskStatus};

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
        metrics: Arc<TaskPoolMetrics>,
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

                let span = tracing::debug_span!(target: "ayr::task::thread", "worker", thread_id = %thread_id);
                let _enter = span.enter();
                tracing::debug!(target: "ayr::task::thread", "starting");
                metrics.threads().record_spawned();

                loop {
                    let status = match commands.recv() {
                        Ok(Command::Stop(_)) | Err(_) => break,
                        Ok(Command::Spawn(timestamp, job)) => {
                            metrics.tasks().record_spawned();
                            metrics
                                .latency()
                                .record_spawn_time(std::time::Instant::now() - timestamp);
                            job.run()
                        },
                        Ok(Command::Tick(timestamp, job)) => {
                            metrics
                                .latency()
                                .record_spawn_time(std::time::Instant::now() - timestamp);
                            job.run()
                        }
                    };

                    if status == TaskStatus::Complete {
                        metrics.tasks().record_completed();
                    }
                }

                tracing::debug!(target: "ayr::task::thread", "exiting");
                metrics.threads().record_dropped();
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
