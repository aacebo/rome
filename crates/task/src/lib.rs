mod cancel;
mod error;
mod execute;
mod pool;
mod result;
mod source;
mod state;
mod status;
mod thread;

pub use cancel::*;
pub use error::*;
pub use execute::*;
pub use pool::*;
pub use result::*;
#[allow(unused)]
pub use source::*;
pub use state::*;
pub use status::*;
pub use thread::*;

use std::{
    sync::{Arc, atomic::Ordering},
    task::Wake,
};

// static GLOBAL: OnceLock<Arc<Executor>> = OnceLock::new();

// fn global_runner() -> Arc<Executor> {
//     GLOBAL
//         .get_or_init(|| Arc::new(Executor::start()))
//         .clone()
// }

pub trait Job: Send + Sync + 'static {
    fn task_id(&self) -> TaskId;
    fn run(self: std::sync::Arc<Self>);
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
    state: Arc<TaskState<T>>,
}

impl<T> Task<T>
where
    T: Send + 'static,
{
    pub fn is_complete(&self) -> bool {
        self.state.status.load(Ordering::Acquire) == TaskStatus::Complete
    }

    pub fn is_cancelled(&self) -> bool {
        self.state.aborted.load(Ordering::Acquire)
    }

    pub fn cancel(&self) {
        self.state.aborted.store(true, Ordering::Release);
        self.state.wake_by_ref();
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
        if self.state.status.load(Ordering::Acquire) == TaskStatus::Complete {
            if self.state.aborted.load(Ordering::Acquire) {
                return std::task::Poll::Ready(Err(TaskError::Cancelled));
            }

            let value = self
                .state
                .output
                .lock()
                .unwrap()
                .take()
                .expect("attempted to join task after output was already consumed");

            return std::task::Poll::Ready(Ok(value));
        }

        *self.state.join.lock().unwrap() = Some(cx.waker().clone());

        if self.state.status.load(Ordering::Acquire) == TaskStatus::Complete {
            cx.waker().wake_by_ref();
        }

        std::task::Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_have_value() {
        use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

        let _ = tracing_subscriber::fmt()
            // .with_test_writer()
            .with_env_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace")),
            )
            .with_span_events(FmtSpan::CLOSE)
            .with_thread_names(true)
            .try_init();

        let ex = Executor::new();
        ex.pool("main");
        ex.start();
        let task = ex.spawn("main", async { 12 });
        let out = task.await.unwrap();
        ex.stop();
        assert_eq!(out, 12)
    }
}
