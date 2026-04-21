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
    /// Get the current clock time.
    fn tick(&self) -> Tick;

    /// Advances the clock by the given wall-clock delta and returns the tick
    /// decision for the current runtime step.
    fn advance_by(&mut self, delta: std::time::Duration) -> Tick;

    /// Wait until the end of the current tick.
    fn wait(&self) {
        let tick = self.tick();
        let spent = tick
            .started_at
            .elapsed()
            .unwrap_or(std::time::Duration::ZERO);
        std::thread::sleep(tick.rate.duration().saturating_sub(spent));
    }
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
    pub rate: Rate,

    /// The amount of time till the next tick.
    pub duration: std::time::Duration,

    /// The start time of the tick.
    pub started_at: std::time::SystemTime,
}

impl Default for Tick {
    fn default() -> Self {
        Self {
            id: TickId::default(),
            steps: 0,
            rate: Rate::Period(std::time::Duration::ZERO),
            duration: std::time::Duration::ZERO,
            started_at: std::time::SystemTime::now(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Rate {
    Hz(u64),
    Period(std::time::Duration),
}

impl Rate {
    pub fn duration(&self) -> std::time::Duration {
        match self {
            Self::Period(v) => *v,
            Self::Hz(v) => std::time::Duration::from_nanos(1_000_000_000 / v),
        }
    }
}

impl From<u64> for Rate {
    fn from(value: u64) -> Self {
        assert!(value > 0);
        Self::Hz(value)
    }
}

impl From<std::time::Duration> for Rate {
    fn from(value: std::time::Duration) -> Self {
        Self::Period(value)
    }
}

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hz(v) => write!(f, "{}Hz", v),
            Self::Period(v) => write!(f, "{:?}", v),
        }
    }
}
