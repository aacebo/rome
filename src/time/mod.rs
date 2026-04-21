mod fixed;

pub use fixed::*;

/// Drives simulation tick timing for the runtime.
///
/// A `Clock` decides when the engine should advance simulation and by how
/// much. Implementations may use a fixed timestep, variable timestep, capped
/// catch-up, pause-aware timing, or any other custom policy.
///
/// This trait is intended to be implemented by engine developers who want
/// control over runtime tick behavior.
pub trait Clock: Send + 'static {
    /// Advances the clock by the given wall-clock delta and returns the tick
    /// decision for the current runtime step.
    fn advance_by(&mut self, delta: std::time::Duration) -> Tick;
}

/// Identifies a discrete step in world simulation time.
///
/// A `Tick` represents an ordered unit of simulation progression within a
/// world or runtime. Unlike wall-clock time, ticks are logical time steps
/// used to sequence deterministic updates, actions, commands, and other
/// engine activity.
///
/// Ticks are monotonically increasing and are typically advanced once for
/// each completed simulation step.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(transparent)]
pub struct TickId(u64);

impl TickId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

/// Describes how the runtime should advance simulation for a step.
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Tick {
    /// The ticks unique sequencial identifier.
    pub id: TickId,

    /// The number of simulation ticks to run now.
    pub steps: u32,

    /// The simulation timestep to use for each tick.
    pub timestep: std::time::Duration,

    /// The amount of time till the next tick.
    pub next: std::time::Duration,

    /// The start time of the tick.
    pub started_at: std::time::SystemTime,

    /// The end time of the tick (or None if still running).
    pub ended_at: Option<std::time::SystemTime>,
}

impl Tick {
    pub fn end(mut self) -> Self {
        self.ended_at = Some(std::time::SystemTime::now());
        self
    }

    pub fn elapsed(&self) -> Option<std::time::Duration> {
        if let Some(ended_at) = &self.ended_at {
            return Some(
                ended_at
                    .duration_since(self.started_at)
                    .expect("started_at must be earlier than ended_at"),
            );
        }

        None
    }

    pub fn wait(&self) {
        std::thread::sleep(self.next);
    }
}
