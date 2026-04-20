use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Clone)]
pub struct CancelToken(Arc<CancelState>);

impl CancelToken {
    pub fn is_cancelled(&self) -> bool {
        self.0.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CancelSource(Arc<CancelState>);

impl CancelSource {
    pub fn new() -> Self {
        Self(Arc::new(CancelState::default()))
    }

    pub fn token(&self) -> CancelToken {
        CancelToken(self.0.clone())
    }

    pub fn cancel(&self) {
        self.0.cancelled.store(true, Ordering::Release);
    }
}

#[derive(Debug, Default)]
struct CancelState {
    cancelled: AtomicBool,
}
