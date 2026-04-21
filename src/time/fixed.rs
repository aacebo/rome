use crate::time::{Clock, Tick, TickId};
use std::time::Duration;

pub struct Fixed {
    next: TickId,
    max: u32,
    timestep: Duration,
    accumulator: Duration,
}

impl Fixed {
    pub fn from_hz(hz: u64) -> Self {
        let step = Duration::from_nanos(1_000_000_000 / hz);
        Self::from_timestep(step)
    }

    pub fn from_timestep(timestep: Duration) -> Self {
        Self {
            next: TickId::default(),
            max: 5,
            timestep,
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
        Self::from_hz(60)
    }
}

impl Clock for Fixed {
    fn advance_by(&mut self, delta: Duration) -> Tick {
        self.accumulator += delta;
        let mut steps = 0;

        while self.accumulator >= self.timestep && steps < self.max {
            self.accumulator -= self.timestep;
            steps += 1;
        }

        if steps >= self.max && self.accumulator >= self.timestep {
            self.accumulator = Duration::ZERO; // anti spiral-of-death
        }

        let id = self.next;
        self.next = self.next.next();

        Tick {
            id,
            steps,
            timestep: self.timestep,
            next: self.timestep.saturating_sub(self.accumulator),
            started_at: std::time::SystemTime::now(),
            ended_at: None,
        }
    }
}
