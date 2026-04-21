use crate::time::{Clock, Rate, Tick};
use std::time::Duration;

pub struct Fixed {
    tick: Tick,
    max: u32,
    rate: Rate,
    accumulator: Duration,
}

impl Fixed {
    pub fn new(rate: impl Into<Rate>) -> Self {
        Self {
            tick: Tick::default(),
            max: 5,
            rate: rate.into(),
            accumulator: Duration::ZERO,
        }
    }

    pub fn with_max(mut self, n: u32) -> Self {
        self.max = n;
        self
    }
}

impl Default for Fixed {
    fn default() -> Self {
        Self::new(60)
    }
}

impl Clock for Fixed {
    fn tick(&self) -> Tick {
        self.tick
    }

    fn advance_by(&mut self, delta: Duration) -> Tick {
        self.accumulator += delta;
        let mut steps = 0;
        let rate = self.rate.duration();

        while self.accumulator >= rate && steps < self.max {
            self.accumulator -= rate;
            steps += 1;
        }

        if steps >= self.max && self.accumulator >= rate {
            self.accumulator = Duration::ZERO; // anti spiral-of-death
        }

        self.tick = Tick {
            id: self.tick.id.next(),
            steps,
            rate: self.rate,
            duration: rate.saturating_sub(self.accumulator),
            started_at: std::time::SystemTime::now(),
        };

        self.tick
    }
}
