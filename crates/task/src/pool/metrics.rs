use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct TaskPoolMetrics {
    _tasks: TaskMetrics,
    _threads: ThreadMetrics,
    _total_latency_ns: AtomicU64,
}

impl TaskPoolMetrics {
    pub fn new() -> Self {
        Self {
            _tasks: TaskMetrics::new(),
            _threads: ThreadMetrics::new(),
            _total_latency_ns: AtomicU64::new(0),
        }
    }

    pub fn tasks(&self) -> &TaskMetrics {
        &self._tasks
    }

    pub fn threads(&self) -> &ThreadMetrics {
        &self._threads
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
            .field("tasks", &self.tasks())
            .field("threads", &self.threads())
            .finish()
    }
}

pub struct TaskMetrics {
    _spawned: AtomicU64,
    _queued: AtomicUsize,
    _completed: AtomicU64,
}

impl TaskMetrics {
    pub fn new() -> Self {
        Self {
            _spawned: AtomicU64::new(0),
            _queued: AtomicUsize::new(0),
            _completed: AtomicU64::new(0),
        }
    }

    pub fn spawned(&self) -> u64 {
        self._spawned.load(Ordering::Acquire)
    }

    pub fn queued(&self) -> usize {
        self._queued.load(Ordering::Acquire)
    }

    pub fn completed(&self) -> u64 {
        self._completed.load(Ordering::Acquire)
    }

    pub fn active(&self) -> u64 {
        self.spawned() - self.completed()
    }

    pub fn record_spawned(&self) {
        self._spawned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_queued(&self) {
        self._queued.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_completed(&self) {
        self._completed.fetch_add(1, Ordering::Relaxed);
    }
}

impl std::fmt::Debug for TaskMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskMetrics")
            .field("spawned", &self.spawned())
            .field("queued", &self.queued())
            .field("complete", &self.completed())
            .finish()
    }
}

pub struct ThreadMetrics {
    _spawned: AtomicU64,
    _dropped: AtomicU64,
}

impl ThreadMetrics {
    pub fn new() -> Self {
        Self {
            _spawned: AtomicU64::new(0),
            _dropped: AtomicU64::new(0),
        }
    }

    pub fn spawned(&self) -> u64 {
        self._spawned.load(Ordering::Acquire)
    }

    pub fn dropped(&self) -> u64 {
        self._dropped.load(Ordering::Acquire)
    }

    pub fn active(&self) -> u64 {
        self.spawned() - self.dropped()
    }

    pub fn record_spawned(&self) {
        self._spawned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_dropped(&self) {
        self._dropped.fetch_add(1, Ordering::Relaxed);
    }
}

impl std::fmt::Debug for ThreadMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThreadMetrics")
            .field("spawned", &self.spawned())
            .field("dropped", &self.dropped())
            .finish()
    }
}
