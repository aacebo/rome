use std::sync::atomic::{AtomicU64, Ordering};

pub struct Version(AtomicU64);

impl Version {
    pub fn to_u64(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    pub fn increment(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self(AtomicU64::new(value))
    }
}

impl Default for Version {
    fn default() -> Self {
        Self(AtomicU64::new(0))
    }
}

impl Eq for Version {}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.to_u64() == other.to_u64()
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_u64().cmp(&other.to_u64())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u64())
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u64())
    }
}
