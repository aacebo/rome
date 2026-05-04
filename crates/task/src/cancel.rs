use std::sync::Arc;

use crate::Async;

#[derive(Clone)]
pub struct Cancellation {
    task: Arc<dyn Async>,
}

impl Cancellation {
    pub(crate) fn from(task: Arc<dyn Async>) -> Self {
        Self { task }
    }

    pub fn is_cancelled(&self) -> bool {
        self.task.is_cancelled()
    }

    pub fn cancel(&self) {
        if !self.task.is_cancelled() {
            self.task.cancel();
        }
    }
}
