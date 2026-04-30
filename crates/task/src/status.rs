use std::sync::atomic::{AtomicU8, Ordering};

#[repr(u8)]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskStatus {
    #[default]
    Parked,
    Queued,
    Running,
    Complete,
}

impl std::fmt::Debug for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parked => write!(f, "parked"),
            Self::Queued => write!(f, "queued"),
            Self::Running => write!(f, "running"),
            Self::Complete => write!(f, "complete"),
        }
    }
}

impl From<u8> for TaskStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Parked,
            1 => Self::Queued,
            2 => Self::Running,
            3 => Self::Complete,
            _ => unreachable!("invalid TaskStatus discriminant"),
        }
    }
}

pub struct AtomicTaskStatus(AtomicU8);

impl AtomicTaskStatus {
    pub const fn new(value: TaskStatus) -> Self {
        Self(AtomicU8::new(value as u8))
    }

    pub fn get(&self) -> TaskStatus {
        self.load(Ordering::Acquire).into()
    }

    pub fn set(&self, value: TaskStatus) {
        self.0.store(value as u8, Ordering::Release);
    }

    pub fn load(&self, order: Ordering) -> TaskStatus {
        self.0.load(order).into()
    }

    pub fn store(&self, value: TaskStatus, order: Ordering) {
        self.0.store(value as u8, order);
    }

    pub fn swap(&self, value: TaskStatus, order: Ordering) -> TaskStatus {
        self.0.swap(value as u8, order).into()
    }

    pub fn compare_exchange(
        &self,
        curr: TaskStatus,
        next: TaskStatus,
        success: Ordering,
        failure: Ordering,
    ) -> Result<TaskStatus, TaskStatus> {
        self.0
            .compare_exchange(curr as u8, next as u8, success, failure)
            .map(|v| v.into())
            .map_err(|v| v.into())
    }
}
