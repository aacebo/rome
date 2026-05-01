use std::sync::atomic::{AtomicU64, Ordering};

pub struct TaskPoolMetrics {
    _tasks: TaskMetrics,
    _threads: ThreadMetrics,
    _latency: LatencyMetrics,
}

impl TaskPoolMetrics {
    pub fn new() -> Self {
        Self {
            _tasks: TaskMetrics::new(),
            _threads: ThreadMetrics::new(),
            _latency: LatencyMetrics::new(),
        }
    }

    pub fn tasks(&self) -> &TaskMetrics {
        &self._tasks
    }

    pub fn threads(&self) -> &ThreadMetrics {
        &self._threads
    }

    pub fn latency(&self) -> &LatencyMetrics {
        &self._latency
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
            .field("latency", &self.latency())
            .finish()
    }
}

pub struct TaskMetrics {
    _queued: AtomicU64,
    _spawned: AtomicU64,
    _completed: AtomicU64,
}

impl TaskMetrics {
    pub fn new() -> Self {
        Self {
            _queued: AtomicU64::new(0),
            _spawned: AtomicU64::new(0),
            _completed: AtomicU64::new(0),
        }
    }

    pub fn queued(&self) -> u64 {
        self._queued.load(Ordering::Acquire)
    }

    pub fn spawned(&self) -> u64 {
        self._spawned.load(Ordering::Acquire)
    }

    pub fn completed(&self) -> u64 {
        self._completed.load(Ordering::Acquire)
    }

    pub fn active(&self) -> u64 {
        self.spawned() - self.completed()
    }

    pub fn record_queued(&self) {
        self._queued.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_spawned(&self) {
        self._spawned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_completed(&self) {
        self._completed.fetch_add(1, Ordering::Relaxed);
    }
}

impl std::fmt::Debug for TaskMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskMetrics")
            .field("queued", &self.queued())
            .field("spawned", &self.spawned())
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

pub struct LatencyMetrics {
    _spawn_ns: AtomicU64,
    _spawn_samples: AtomicU64,
}

impl LatencyMetrics {
    pub fn new() -> Self {
        Self {
            _spawn_ns: AtomicU64::new(0),
            _spawn_samples: AtomicU64::new(0),
        }
    }

    pub fn spawn_time(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.spawn_ns())
    }

    pub fn spawn_ns(&self) -> u64 {
        self._spawn_ns.load(Ordering::Acquire)
    }

    pub fn avg_spawn_time(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.avg_spawn_ns())
    }

    pub fn avg_spawn_ns(&self) -> u64 {
        let total = self._spawn_ns.load(Ordering::Relaxed);
        let samples = self._spawn_samples.load(Ordering::Relaxed);

        if samples == 0 {
            return 0;
        }

        total / samples
    }

    pub fn record_spawn_time(&self, value: std::time::Duration) {
        self._spawn_samples.fetch_add(1, Ordering::Release);
        self._spawn_ns.fetch_add(
            value.as_nanos().min(u64::MAX as u128) as u64,
            Ordering::Relaxed,
        );
    }
}

impl std::fmt::Debug for LatencyMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LatencyMetrics")
            .field("spawn_time", &self.spawn_time())
            .field("avg_spawn_time", &self.avg_spawn_time())
            .finish()
    }
}
