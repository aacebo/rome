use std::sync::atomic::{AtomicU128, Ordering};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct AtomicInstant {
    base: Instant,
    offset_ns: AtomicU128,
}

impl AtomicInstant {
    pub fn new(instant: Instant) -> Self {
        Self {
            base: instant,
            offset_ns: AtomicU128::new(0),
        }
    }

    pub fn now() -> Self {
        Self::new(Instant::now())
    }

    pub fn get(&self) -> Instant {
        self.load(Ordering::Acquire)
    }

    pub fn set(&self, value: Instant) {
        self.store(value, Ordering::Release)
    }

    pub fn load(&self, ordering: Ordering) -> Instant {
        let ns = self.offset_ns.load(ordering);
        self.base + Duration::from_nanos_u128(ns)
    }

    pub fn store(&self, instant: Instant, ordering: Ordering) {
        let offset = instant.duration_since(self.base);
        self.offset_ns.store(offset.as_nanos(), ordering);
    }

    pub fn swap(&self, instant: Instant, ordering: Ordering) -> Instant {
        let offset = instant.duration_since(self.base);
        let old = self.offset_ns.swap(offset.as_nanos(), ordering);
        self.base + Duration::from_nanos_u128(old)
    }

    pub fn compare_exchange(
        &self,
        current: Instant,
        new: Instant,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Instant, Instant> {
        match self.offset_ns.compare_exchange(
            current.duration_since(self.base).as_nanos(),
            new.duration_since(self.base).as_nanos(),
            success,
            failure,
        ) {
            Ok(old) => Ok(self.base + Duration::from_nanos_u128(old)),
            Err(actual) => Err(self.base + Duration::from_nanos_u128(actual)),
        }
    }

    pub fn elapsed(&self, ordering: Ordering) -> Duration {
        self.load(ordering).elapsed()
    }

    pub fn duration_since(&self, earlier: Instant, ordering: Ordering) -> Duration {
        self.load(ordering).duration_since(earlier)
    }
}
