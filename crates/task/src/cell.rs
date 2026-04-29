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

use crate::{AtomicTaskState, Run, TaskState};

pub struct TaskCell<T> {
    pub(crate) state: AtomicTaskState,
    pub(crate) aborted: AtomicBool,
    pub(crate) join: Mutex<Option<Waker>>,
    pub(crate) sender: crossbeam::channel::Sender<Arc<dyn Run>>,
    pub(crate) output: Mutex<Option<T>>,
    pub(crate) future: Mutex<Option<BoxFuture<'static, T>>>,
}

impl<T> Wake for TaskCell<T>
where
    T: Send + 'static,
{
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        // if complete do nothing
        if self.state.load(Ordering::Acquire) == TaskState::Complete {
            return;
        }

        // mark as queued, if previous state was not queued
        // then queue the task
        if self.state.swap(TaskState::Queued, Ordering::AcqRel) != TaskState::Queued {
            let _ = self.sender.send(self.clone());
        }
    }
}

impl<T> ArcWake for TaskCell<T>
where
    T: Send + 'static,
{
    fn wake_by_ref(task: &Arc<Self>) {
        // if complete do nothing
        if task.state.load(Ordering::Acquire) == TaskState::Complete {
            return;
        }

        // mark as queued, if previous state was not queued
        // then queue the task
        if task.state.swap(TaskState::Queued, Ordering::AcqRel) != TaskState::Queued {
            let _ = task.sender.send(task.clone());
        }
    }
}

impl<T> Run for TaskCell<T>
where
    T: Send + 'static,
{
    fn run(self: std::sync::Arc<Self>) {
        let state = self.state.swap(TaskState::Running, Ordering::AcqRel);

        // if complete do nothing
        if state == TaskState::Complete {
            self.state.store(TaskState::Complete, Ordering::Release);
            return;
        }

        if self.aborted.load(Ordering::Acquire) {
            *self.future.lock().unwrap() = None;
            self.state.store(TaskState::Complete, Ordering::Release);

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
                if let Some(waker) = self.join.lock().unwrap().as_ref() {
                    waker.wake_by_ref();
                }
            }
            Poll::Ready(value) => {
                *slot = None;
                *self.output.lock().unwrap() = Some(value);
                self.state.store(TaskState::Complete, Ordering::Release);

                if let Some(waker) = self.join.lock().unwrap().take() {
                    waker.wake();
                }
            }
        }
    }
}
