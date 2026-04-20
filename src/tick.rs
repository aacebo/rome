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

/// Configures how the engine advances simulation ticks.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TickSettings {
    /// The timing mode used by the engine.
    pub mode: TickMode,

    /// Maximum number of catch-up ticks allowed in a single frame.
    pub max_ticks_per_frame: u32,
}

/// Describes the timing policy used to advance the engine.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TickMode {
    /// Runs simulation at a fixed rate.
    Fixed {
        /// Number of ticks per second.
        rate_hz: u32,
    },

    /// Runs simulation using the measured frame delta.
    Variable,

    /// Runs simulation as fast as possible.
    Unbounded,
}
