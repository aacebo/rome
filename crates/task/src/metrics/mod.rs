mod latency;
mod pool;
mod task;
mod thread;

pub use latency::*;
pub use pool::*;
pub use task::*;
pub use thread::*;

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct MetricU64(AtomicU64);

impl MetricU64 {
    pub fn new(value: u64) -> Self {
        Self(AtomicU64::new(value))
    }

    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    pub fn set(&self, value: u64) {
        self.0.store(value, Ordering::Release)
    }

    pub fn add(&self, value: u64) {
        self.0.fetch_add(value, Ordering::Relaxed);
    }

    pub fn increment(&self) {
        self.add(1)
    }

    pub fn sub(&self, value: u64) {
        self.0.update(Ordering::SeqCst, Ordering::SeqCst, |curr| {
            curr.saturating_sub(value)
        });
    }

    pub fn decrement(&self) {
        self.sub(1);
    }
}

impl Default for MetricU64 {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<u64> for MetricU64 {
    fn from(value: u64) -> Self {
        Self(AtomicU64::new(value))
    }
}

impl std::ops::Deref for MetricU64 {
    type Target = AtomicU64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for MetricU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl std::fmt::Debug for MetricU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub struct MetricUSize(AtomicUsize);

impl MetricUSize {
    pub fn new(value: usize) -> Self {
        Self(AtomicUsize::new(value))
    }

    pub fn get(&self) -> usize {
        self.0.load(Ordering::Acquire)
    }

    pub fn set(&self, value: usize) {
        self.0.store(value, Ordering::Release)
    }

    pub fn add(&self, value: usize) {
        self.0.fetch_add(value, Ordering::Relaxed);
    }

    pub fn sub(&self, value: usize) {
        self.0.update(Ordering::SeqCst, Ordering::SeqCst, |curr| {
            curr.saturating_sub(value)
        });
    }
}

impl Default for MetricUSize {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<usize> for MetricUSize {
    fn from(value: usize) -> Self {
        Self(AtomicUsize::new(value))
    }
}

impl std::ops::Deref for MetricUSize {
    type Target = AtomicUsize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for MetricUSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl std::fmt::Debug for MetricUSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}
