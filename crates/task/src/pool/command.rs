use std::sync::Arc;

use crate::Job;

pub enum Command {
    Stop(std::time::Instant),
    Spawn(std::time::Instant, Arc<dyn Job>),
    Tick(std::time::Instant, Arc<dyn Job>),
}

impl Command {
    pub fn stop() -> Self {
        Self::Stop(std::time::Instant::now())
    }

    pub fn spawn(job: Arc<dyn Job>) -> Self {
        Self::Spawn(std::time::Instant::now(), job)
    }

    pub fn tick(job: Arc<dyn Job>) -> Self {
        Self::Tick(std::time::Instant::now(), job)
    }
}
