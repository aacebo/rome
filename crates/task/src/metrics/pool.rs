use crate::metrics::{TaskMetrics, TaskMetricsSnapshot, ThreadMetrics, ThreadMetricsSnapshot};

#[derive(Debug, Default)]
pub struct PoolMetrics {
    pub tasks: TaskMetrics,
    pub threads: ThreadMetrics,
}

impl PoolMetrics {
    pub fn queue_depth_per_worker(&self) -> f64 {
        self.tasks.in_queue() as f64 / self.threads.active.get() as f64
    }

    pub fn utilization(&self) -> f64 {
        let max_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        self.threads.active.get() as f64 / max_threads as f64
    }

    pub fn snapshot(&self) -> PoolMetricsSnapshot {
        PoolMetricsSnapshot {
            tasks: self.tasks.snapshot(),
            threads: self.threads.snapshot(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PoolMetricsSnapshot {
    pub tasks: TaskMetricsSnapshot,
    pub threads: ThreadMetricsSnapshot,
}
