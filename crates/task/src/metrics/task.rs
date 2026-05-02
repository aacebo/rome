use super::{MetricLatency, MetricU64};

#[derive(Debug, Default)]
pub struct TaskMetrics {
    pub queued: MetricU64,
    pub spawned: MetricU64,
    pub completed: MetricU64,
    pub spawn_latency_ns: MetricLatency,
}

impl TaskMetrics {
    pub fn in_queue(&self) -> u64 {
        self.queued.get() - self.spawned.get()
    }

    pub fn active(&self) -> u64 {
        self.spawned.get() - self.completed.get()
    }

    pub fn snapshot(&self) -> TaskMetricsSnapshot {
        TaskMetricsSnapshot {
            queued: self.queued.get(),
            spawned: self.spawned.get(),
            completed: self.completed.get(),
            in_queue: self.in_queue(),
            active: self.active(),
            avg_spawn_latency: std::time::Duration::from_nanos(self.spawn_latency_ns.get()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TaskMetricsSnapshot {
    pub queued: u64,
    pub spawned: u64,
    pub completed: u64,
    pub in_queue: u64,
    pub active: u64,
    pub avg_spawn_latency: std::time::Duration,
}
