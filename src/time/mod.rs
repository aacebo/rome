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
    fn tick(&mut self, delta: std::time::Duration) -> TickDecision;
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
pub struct Tick(u64);

impl Tick {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

/// Describes how the runtime should advance simulation for a step.
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TickDecision {
    /// The number of simulation ticks to run now.
    pub ticks: u32,

    /// The simulation timestep to use for each tick.
    pub timestep: std::time::Duration,
}
