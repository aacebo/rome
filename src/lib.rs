pub mod context;
pub mod diagnostic;
pub mod entity;
pub mod event;
pub mod math;
pub mod meta;
mod tick;
pub mod world;

pub use tick::*;

/// A Layer represents a logical world system.
///
/// Modules are systems that coordinate entities in a world and
/// drive more complex multi-entity logic (ex. Phsyics).
pub trait Layer: Send + Sync + 'static {
    fn on_start(&self, _ctx: &mut context::Context) {}
    fn on_tick(&self, _ctx: &mut context::Context) {}
    fn on_stop(&self, _ctx: &mut context::Context) {}
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

    fn on_create(&mut self, _ctx: &mut context::EntityContext) {}
    fn on_update(&mut self, _ctx: &mut context::EntityContext) {}
    fn on_delete(&mut self, _ctx: &mut context::EntityContext) {}
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
