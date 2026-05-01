use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::{Event, TaskEvent, ThreadEvent};

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

    pub fn reduce(&self, event: &Event) {
        match event {
            Event::Task(event) => match event {
                TaskEvent::Queued(_) => {
                    self.tasks_queued.fetch_add(1, Ordering::Relaxed);
                }
                TaskEvent::Completed(_) => {
                    self.tasks_completed.fetch_add(1, Ordering::Relaxed);
                }
                TaskEvent::Spawned(_) => {
                    self.tasks_spawned.fetch_add(1, Ordering::Relaxed);
                }
            },
            Event::Thread(event) => match event {
                ThreadEvent::Stopped(_) => {
                    self.threads_idle.fetch_add(1, Ordering::Relaxed);
                    self.threads_active.fetch_sub(1, Ordering::Relaxed);
                }
                ThreadEvent::Spawned(_) => {
                    self.threads_idle.fetch_sub(1, Ordering::Relaxed);
                    self.threads_active.fetch_add(1, Ordering::Relaxed);
                }
            },
        };
    }
}

impl Default for TaskPoolMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for TaskPoolMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskPoolMetrics")
            .field("tasks_queued", &self.tasks_queued())
            .field("tasks_completed", &self.tasks_completed())
            .field("tasks_spawned", &self.tasks_spawned())
            .field("threads_idle", &self.threads_idle())
            .field("threads_active", &self.threads_active())
            .finish()
    }
}
