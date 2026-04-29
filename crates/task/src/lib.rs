mod cancel;
mod cell;
mod error;
mod execute;
mod join;
mod result;
mod source;
mod state;

pub use cancel::*;
pub use cell::*;
pub use error::*;
pub use execute::*;
pub use join::*;
pub use result::*;
pub use source::*;
pub use state::*;

use std::{
    sync::{Arc, OnceLock, atomic::Ordering},
    task::Wake,
};

// static GLOBAL: OnceLock<Arc<Executor>> = OnceLock::new();

// fn global_runner() -> Arc<Executor> {
//     GLOBAL
//         .get_or_init(|| Arc::new(Executor::start()))
//         .clone()
// }

trait Run: Send + Sync + 'static {
    fn run(self: std::sync::Arc<Self>);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(u64);

impl TaskId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

pub struct Task<T> {
    cell: Arc<TaskCell<T>>,
    receiver: crossbeam::channel::Receiver<Arc<dyn Run>>,
}

impl<T> Task<T>
where
    T: Send + 'static,
{
    pub fn is_complete(&self) -> bool {
        self.cell.state.load(Ordering::Acquire) == TaskState::Complete
    }

    pub fn is_cancelled(&self) -> bool {
        self.cell.aborted.load(Ordering::Acquire)
    }

    pub fn cancel(&self) {
        self.cell.aborted.store(true, Ordering::Release);
        self.cell.wake_by_ref();
    }
}

impl<T> Future for Task<T>
where
    T: Send + 'static,
{
    type Output = Result<T, TaskError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.cell.state.load(Ordering::Acquire) == TaskState::Complete {
            if self.cell.aborted.load(Ordering::Acquire) {
                return std::task::Poll::Ready(Err(TaskError::Cancelled));
            }

            let value = self
                .cell
                .output
                .lock()
                .unwrap()
                .take()
                .expect("attempted to join task after output was already consumed");

            return std::task::Poll::Ready(Ok(value));
        }

        *self.cell.join.lock().unwrap() = Some(cx.waker().clone());

        if self.cell.state.load(Ordering::Acquire) == TaskState::Complete {
            cx.waker().wake_by_ref();
        }

        match self.receiver.try_recv() {
            Err(crossbeam::channel::TryRecvError::Empty) => std::task::Poll::Pending,
            Err(crossbeam::channel::TryRecvError::Disconnected) => {
                std::task::Poll::Ready(Err(TaskError::Dropped))
            }
            Ok(v) => {
                v.run();
                std::task::Poll::Pending
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_have_value() {
        let ex = Executor::new();
        let task = ex.spawn(async { 12 });
        let out = task.await.unwrap();
        assert_eq!(out, 12)
    }
}
