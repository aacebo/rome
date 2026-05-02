use crate::metrics::MetricU64;

#[derive(Debug, Default)]
pub struct ThreadMetrics {
    pub spawned: MetricU64,
    pub dropped: MetricU64,
    pub active: MetricU64,
    pub idle: MetricU64,
}

impl ThreadMetrics {
    pub fn snapshot(&self) -> ThreadMetricsSnapshot {
        ThreadMetricsSnapshot {
            spawned: self.spawned.get(),
            dropped: self.dropped.get(),
            active: self.active.get(),
            idle: self.idle.get(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ThreadMetricsSnapshot {
    pub spawned: u64,
    pub dropped: u64,
    pub active: u64,
    pub idle: u64,
}
