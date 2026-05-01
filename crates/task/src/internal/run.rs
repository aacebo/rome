use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

use futures::{
    FutureExt,
    future::BoxFuture,
    task::{ArcWake, waker_ref},
};

use crate::{AtomicTaskStatus, Command, Job, TaskId, TaskPoolMetrics, TaskStatus};

pub(crate) struct TaskRun<T> {
    id: TaskId,
    status: AtomicTaskStatus,
    aborted: AtomicBool,
    waker: Mutex<Option<Waker>>,
    output: Mutex<Option<T>>,
    future: Mutex<Option<BoxFuture<'static, T>>>,
    metrics: Arc<TaskPoolMetrics>,
    commands: crossbeam::channel::Sender<Command>,
}

impl<T> TaskRun<T>
where
    T: Send + 'static,
{
    pub fn new(
        id: TaskId,
        metrics: Arc<TaskPoolMetrics>,
        commands: crossbeam::channel::Sender<Command>,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Self {
        TaskRun {
            id,
            status: AtomicTaskStatus::new(TaskStatus::default()),
            aborted: AtomicBool::new(false),
            waker: Mutex::new(None),
            output: Mutex::new(None),
            future: Mutex::new(Some(future.boxed())),
            metrics,
            commands,
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.aborted.load(Ordering::Acquire)
    }

    pub fn status(&self) -> TaskStatus {
        self.status.get()
    }

    pub fn output(&self) -> Option<T> {
        self.output.lock().unwrap().take()
    }

    pub fn register(&self, waker: Waker) {
        *self.waker.lock().unwrap() = Some(waker);
    }

    pub fn complete(&self, value: T) {
        *self.output.lock().unwrap() = Some(value);
        self.metrics.record_completed();
        self.status.store(TaskStatus::Complete, Ordering::Release);
    }

    pub fn cancel(&self) {
        self.aborted.store(true, Ordering::Release);
        self.metrics.record_completed();
    }
}

impl<T> Wake for TaskRun<T>
where
    T: Send + 'static,
{
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        tracing::trace!(target: "ayr::task", task_id = %self.id, "wake");

        // if complete do nothing
        if self.status() == TaskStatus::Complete {
            return;
        }

        // mark as queued, if previous status was not queued
        // then queue the task
        if self.status.swap(TaskStatus::Queued, Ordering::AcqRel) != TaskStatus::Queued {
            self.metrics.record_queued();
            let _ = self.commands.send(Command::Run(self.clone()));
        }
    }
}

impl<T> ArcWake for TaskRun<T>
where
    T: Send + 'static,
{
    fn wake_by_ref(task: &Arc<Self>) {
        Wake::wake_by_ref(task);
    }
}

impl<T> Job for TaskRun<T>
where
    T: Send + 'static,
{
    fn run(self: std::sync::Arc<Self>) {
        let status = self.status.swap(TaskStatus::Running, Ordering::AcqRel);

        // if complete do nothing
        if status == TaskStatus::Complete {
            self.status.store(TaskStatus::Complete, Ordering::Release);
            return;
        }

        if self.aborted.load(Ordering::Acquire) {
            tracing::debug!(target: "ayr::task", task_id = ?self.id, "cancelled");
            *self.future.lock().unwrap() = None;
            self.status.store(TaskStatus::Complete, Ordering::Release);

            if let Some(waker) = self.waker.lock().unwrap().take() {
                waker.wake();
            }

            return;
        }

        let waker = waker_ref(&self);
        let mut cx = Context::from_waker(&*waker);
        let mut slot = self.future.lock().unwrap();
        let Some(future) = slot.as_mut() else {
            return;
        };

        match future.as_mut().poll(&mut cx) {
            Poll::Pending => {
                tracing::trace!(target: "ayr::task", task_id = %self.id, "pending");

                if let Some(waker) = self.waker.lock().unwrap().as_ref() {
                    waker.wake_by_ref();
                }
            }
            Poll::Ready(value) => {
                tracing::debug!(target: "ayr::task", task_id = %self.id, "ready");
                *slot = None;
                self.complete(value);

                if let Some(waker) = self.waker.lock().unwrap().take() {
                    waker.wake();
                }
            }
        }
    }
}
