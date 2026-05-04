use std::sync::Arc;

use crate::Async;

pub enum Command {
    Stop(std::time::Instant),
    Spawn(std::time::Instant, Arc<dyn Async>),
    Tick(std::time::Instant, Arc<dyn Async>),
}

impl Command {
    pub fn stop() -> Self {
        Self::Stop(std::time::Instant::now())
    }

    pub fn spawn(job: Arc<dyn Async>) -> Self {
        Self::Spawn(std::time::Instant::now(), job)
    }

    pub fn tick(job: Arc<dyn Async>) -> Self {
        Self::Tick(std::time::Instant::now(), job)
    }
}
