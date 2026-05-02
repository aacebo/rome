use std::sync::atomic::{AtomicU64, Ordering};

pub struct MetricLatency {
    ema_ns: AtomicU64,
    samples: AtomicU64,
    alpha_numer: u64,
    alpha_denom: u64,
}

impl MetricLatency {
    pub fn new(alpha_numer: u64, alpha_denom: u64) -> Self {
        assert!(alpha_denom > 0, "alpha_denom must be > 0");
        assert!(
            alpha_numer <= alpha_denom,
            "alpha_numer must be <= alpha_denom"
        );
        Self {
            ema_ns: AtomicU64::new(0),
            samples: AtomicU64::new(0),
            alpha_numer,
            alpha_denom,
        }
    }

    pub fn get(&self) -> u64 {
        self.ema_ns.load(Ordering::Acquire)
    }

    pub fn samples(&self) -> u64 {
        self.samples.load(Ordering::Acquire)
    }

    pub fn add(&self, value_ns: u64) {
        // ema_new = ema + alpha * (value - ema)
        //         = ema + (numer/denom) * (value - ema)
        //
        // To stay in u64 with no signed overflow, compute the delta with
        // saturating arithmetic and choose direction explicitly.
        let mut current = self.ema_ns.load(Ordering::Acquire);

        loop {
            let next = if self.samples.load(Ordering::Relaxed) == 0 {
                value_ns
            } else if value_ns >= current {
                let delta = value_ns - current;
                let step =
                    (delta as u128 * self.alpha_numer as u128 / self.alpha_denom as u128) as u64;
                current.saturating_add(step)
            } else {
                let delta = current - value_ns;
                let step =
                    (delta as u128 * self.alpha_numer as u128 / self.alpha_denom as u128) as u64;
                current.saturating_sub(step)
            };

            match self.ema_ns.compare_exchange_weak(
                current,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(observed) => current = observed,
            }
        }
        self.samples.fetch_add(1, Ordering::Relaxed);
    }
}

impl Default for MetricLatency {
    fn default() -> Self {
        Self::new(1, 8)
    }
}

impl std::fmt::Debug for MetricLatency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}
