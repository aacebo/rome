use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct TaskPoolMetrics {
    tasks_spawned: AtomicU64,
    tasks_queued: AtomicUsize,
    tasks_completed: AtomicU64,
    threads_spawned: AtomicU64,
    threads_dropped: AtomicU64,
    total_latency_ns: AtomicU64,
}

impl TaskPoolMetrics {
    pub fn new() -> Self {
        Self {
            tasks_spawned: AtomicU64::new(0),
            tasks_queued: AtomicUsize::new(0),
            tasks_completed: AtomicU64::new(0),
            threads_spawned: AtomicU64::new(0),
            threads_dropped: AtomicU64::new(0),
            total_latency_ns: AtomicU64::new(0),
        }
    }

    pub fn tasks_spawned(&self) -> u64 {
        self.tasks_spawned.load(Ordering::Acquire)
    }

    pub fn tasks_queued(&self) -> usize {
        self.tasks_queued.load(Ordering::Acquire)
    }

    pub fn tasks_completed(&self) -> u64 {
        self.tasks_completed.load(Ordering::Acquire)
    }

    pub fn tasks_active(&self) -> u64 {
        self.tasks_spawned() - self.tasks_completed()
    }

    pub fn threads_spawned(&self) -> u64 {
        self.threads_spawned.load(Ordering::Acquire)
    }

    pub fn threads_dropped(&self) -> u64 {
        self.threads_dropped.load(Ordering::Acquire)
    }

    pub fn threads_active(&self) -> u64 {
        self.threads_spawned() - self.threads_dropped()
    }

    pub fn total_latency_ns(&self) -> u64 {
        self.total_latency_ns.load(Ordering::Acquire)
    }

    pub fn record_queued(&self) {
        self.tasks_queued.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_completed(&self) {
        self.tasks_completed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_spawned(&self) {
        self.tasks_spawned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_thread_spawned(&self) {
        self.threads_spawned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_thread_dropped(&self) {
        self.threads_dropped.fetch_add(1, Ordering::Relaxed);
    }
}

// impl TaskPoolMetrics {
//     pub fn queue_depth_per_worker(&self) -> f64 {
//         self.tasks_queued() as f64 / self.threads_active() as f64
//     }

//     pub fn utilization(&self) -> f64 {
//         self.threads_active() as f64 / (self.threads_active() + self.threads()) as f64
//     }
// }

impl Default for TaskPoolMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for TaskPoolMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskPoolMetrics")
            .field("tasks_spawned", &self.tasks_spawned())
            .field("tasks_queued", &self.tasks_queued())
            .field("tasks_completed", &self.tasks_completed())
            .field("threads_spawned", &self.threads_spawned())
            .field("threads_dropped", &self.threads_dropped())
            .field("threads_active", &self.threads_active())
            .finish()
    }
}
