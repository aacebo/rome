use std::sync::atomic::{AtomicU64, Ordering};

pub struct AtomicInteger(AtomicU64);

impl AtomicInteger {
    pub fn new(value: u64) -> Self {
        Self(AtomicU64::new(value))
    }

    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    pub fn set(&self, value: u64) {
        self.0.store(value, Ordering::Release)
    }
}

impl From<u64> for AtomicInteger {
    fn from(value: u64) -> Self {
        Self(AtomicU64::new(value))
    }
}

impl Into<u64> for AtomicInteger {
    fn into(self) -> u64 {
        self.get()
    }
}
