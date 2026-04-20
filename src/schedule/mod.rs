mod sequence;

pub use sequence::*;

use crate::{Context, Layer};

/// Schedules world-layer execution for an engine.
///
/// A `Scheduler` defines **how** and **when** layers run during the engine
/// lifecycle. The simplest implementation executes every layer sequentially
/// on each tick, but other schedulers may support phased execution,
/// dependency ordering, fixed timesteps, or parallel dispatch.
///
/// A scheduler does not own world state. It coordinates execution against
/// a mutable [`context::Context`], which provides access to the active world
/// and any engine-scoped services needed during a tick.
pub trait Scheduler: Send + Sync + 'static {
    fn on_start(&mut self, _ctx: &mut Context, _layers: &mut [Box<dyn Layer>]) {}
    fn on_tick(&mut self, ctx: &mut Context, layers: &mut [Box<dyn Layer>]);
    fn on_stop(&mut self, _ctx: &mut Context, _layers: &mut [Box<dyn Layer>]) {}
}
