#![feature(integer_atomics)]

pub mod cancel;
pub mod command;
pub mod config;
pub mod error;
pub mod execute;
pub(crate) mod internal;
pub mod metrics;
pub mod pool;
pub mod prelude;
pub mod status;

pub use command::Command;

use std::sync::Arc;

use crate::{cancel::Cancellation, error::TaskError, status::TaskStatus};

pub trait Async: Send + Sync + 'static {
    fn is_cancelled(&self) -> bool;
    fn status(&self) -> TaskStatus;
    fn cancel(&self);
    fn run(self: std::sync::Arc<Self>) -> TaskStatus;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(u64);

impl TaskId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u64> for TaskId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub struct Task<T> {
    run: Arc<internal::TaskRun<T>>,
}

impl<T> Task<T>
where
    T: Send + 'static,
{
    pub fn cancellation(&self) -> Cancellation {
        Cancellation::from(self.run.clone())
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
        if self.run.status() == TaskStatus::Complete {
            if self.run.is_cancelled() {
                return std::task::Poll::Ready(Err(TaskError::Cancelled));
            }

            let value = self
                .run
                .output()
                .expect("attempted to join task after output was already consumed");

            return std::task::Poll::Ready(Ok(value));
        }

        self.run.register(cx.waker().clone());

        if self.run.status() == TaskStatus::Complete {
            cx.waker().wake_by_ref();
        }

        std::task::Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn should_have_value() {
        let ex = Executor::new();
        let pool = ex.pool(PoolConfig::new("main"));
        let task = ex.spawn("main", async { 12 });
        let out = task.await.unwrap();
        pool.stop();

        println!("{:#?}", pool.metrics());
        assert_eq!(out, 12);
        assert_eq!(pool.metrics().tasks.spawned, 1);
        assert_eq!(pool.metrics().tasks.queued, 1);
        assert_eq!(pool.metrics().tasks.completed, 1);
    }
}
