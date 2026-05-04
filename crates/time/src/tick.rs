use crate::Rate;

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
