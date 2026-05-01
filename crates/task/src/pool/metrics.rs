use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct TaskPoolMetrics {
    tasks_queued: AtomicUsize,
    tasks_completed: AtomicU64,
    tasks_spawned: AtomicU64,
    threads_idle: AtomicUsize,
    threads_active: AtomicUsize,
    total_latency_ns: AtomicU64,
}

impl TaskPoolMetrics {
    pub fn new() -> Self {
        Self {
            tasks_queued: AtomicUsize::new(0),
            tasks_completed: AtomicU64::new(0),
            tasks_spawned: AtomicU64::new(0),
            threads_idle: AtomicUsize::new(0),
            threads_active: AtomicUsize::new(0),
            total_latency_ns: AtomicU64::new(0),
        }
    }

    pub fn tasks_queued(&self) -> usize {
        self.tasks_queued.load(Ordering::Acquire)
    }

    pub fn tasks_completed(&self) -> u64 {
        self.tasks_completed.load(Ordering::Acquire)
    }

    pub fn tasks_spawned(&self) -> u64 {
        self.tasks_spawned.load(Ordering::Acquire)
    }

    pub fn threads_idle(&self) -> usize {
        self.threads_idle.load(Ordering::Acquire)
    }

    pub fn threads_active(&self) -> usize {
        self.threads_active.load(Ordering::Acquire)
    }

    pub fn total_latency_ns(&self) -> u64 {
        self.total_latency_ns.load(Ordering::Acquire)
    }
}

impl Default for TaskPoolMetrics {
    fn default() -> Self {
        Self::new()
    }
}
