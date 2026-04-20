pub mod action;
mod cancel;
mod context;
pub mod diagnostic;
pub mod entity;
pub mod math;
pub mod meta;
mod runtime;
mod tick;
pub mod world;

pub use cancel::*;
pub use context::*;
pub use runtime::*;
pub use tick::*;

use crate::entity::Entity;

/// A Layer represents a logical world system.
///
/// Modules are systems that coordinate entities in a world and
/// drive more complex multi-entity logic (ex. Phsyics).
pub trait Layer: Send + Sync + 'static {
    fn name(&self) -> &str;

    fn on_start(&mut self, _ctx: &mut Context) {}
    fn on_tick(&mut self, _ctx: &mut Context) {}
    fn on_stop(&mut self, _ctx: &mut Context) {}
}

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

/// A Facet represents a single, focused aspect of an Entity's state and behavior.
///
/// Facets are the primary building blocks used to compose Entities. Each Facet
/// should encapsulate one concern (e.g. health, rendering, movement) and avoid
/// depending directly on other Facets.
///
/// Complex interactions between Facets should be coordinated externally
/// (e.g. via Layers) rather than through tight coupling.
///
/// In this model:
/// - Entities provide identity and composition
/// - Facets provide capabilities
pub trait Facet: Send + Sync + 'static {
    fn name(&self) -> &str;

    fn on_create(&mut self, _ctx: &mut Context, _entity: &mut Entity) {}
    fn on_update(&mut self, _ctx: &mut Context, _entity: &mut Entity) {}
    fn on_delete(&mut self, _ctx: &mut Context, _entity: &mut Entity) {}
}

impl serde::Serialize for dyn Facet {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name().serialize(s)
    }
}

impl std::fmt::Debug for dyn Facet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.name())
    }
}
