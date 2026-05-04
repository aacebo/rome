mod fixed;
mod rate;
mod tick;

pub use fixed::*;
pub use rate::*;
pub use tick::*;

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
