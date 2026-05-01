use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

use futures::{
    future::BoxFuture,
    task::{ArcWake, waker_ref},
};

use crate::{AtomicTaskStatus, Job, Message, TaskId, TaskStatus};

pub struct TaskState<T> {
    pub(crate) id: TaskId,
    pub(crate) status: AtomicTaskStatus,
    pub(crate) aborted: AtomicBool,
    pub(crate) join: Mutex<Option<Waker>>,
    pub(crate) sender: crossbeam::channel::Sender<Message>,
    pub(crate) output: Mutex<Option<T>>,
    pub(crate) future: Mutex<Option<BoxFuture<'static, T>>>,
}

impl<T> Wake for TaskState<T>
where
    T: Send + 'static,
{
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        tracing::trace!(target: "ayr::task", task_id = %self.id, "wake");

        // if complete do nothing
        if self.status.load(Ordering::Acquire) == TaskStatus::Complete {
            return;
        }

        // mark as queued, if previous status was not queued
        // then queue the task
        if self.status.swap(TaskStatus::Queued, Ordering::AcqRel) != TaskStatus::Queued {
            let _ = self.sender.send(Message::Job(self.clone()));
        }
    }
}

impl<T> ArcWake for TaskState<T>
where
    T: Send + 'static,
{
    fn wake_by_ref(task: &Arc<Self>) {
        Wake::wake_by_ref(task);
    }
}

impl<T> Job for TaskState<T>
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

            if let Some(waker) = self.join.lock().unwrap().take() {
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

                if let Some(waker) = self.join.lock().unwrap().as_ref() {
                    waker.wake_by_ref();
                }
            }
            Poll::Ready(value) => {
                tracing::debug!(target: "ayr::task", task_id = %self.id, "ready");
                *slot = None;
                *self.output.lock().unwrap() = Some(value);
                self.status.store(TaskStatus::Complete, Ordering::Release);

                if let Some(waker) = self.join.lock().unwrap().take() {
                    waker.wake();
                }
            }
        }
    }
}
