pub mod context;
mod node;
pub mod prelude;
pub mod reflect;
pub mod state;
pub mod view;
pub mod world;

pub use context::Context;
pub use node::*;
pub use reflect::Value;
pub use view::Style;
pub use world::World;

pub trait Scene: Send + Sync + std::any::Any {
    fn name(&self) -> &'static str;

    /// Called when the scene is entered/started.
    fn on_enter(&mut self, _ctx: &Context) {}

    /// Called before rendering.
    fn on_tick(&mut self, _ctx: &Context) {}

    /// Called when the scene is exited/stopped.
    fn on_exit(&mut self, _ctx: &Context) {}
}

pub trait Entity<TScene>: Send + Sync + std::any::Any
where
    TScene: Scene,
{
    fn name(&self) -> &str;

    fn on_spawn(&mut self, _ctx: &Context, _scene: &mut TScene) {}
    fn on_change(&mut self, _ctx: &Context, _scene: &mut TScene) {}
    fn on_destroy(&mut self, _ctx: &Context, _scene: &mut TScene) {}
}

#[doc(hidden)]
pub trait AnyEntity: Send + Sync + std::any::Any {
    fn name(&self) -> &str;

    fn on_spawn(&mut self, _ctx: &Context, _scene: &dyn Scene) {}
    fn on_change(&mut self, _ctx: &Context, _scene: &dyn Scene) {}
    fn on_destroy(&mut self, _ctx: &Context, _scene: &dyn Scene) {}
}

pub trait Attribute<TEntity>: Send + Sync + std::any::Any
where
    TEntity: AnyEntity,
{
    fn name(&self) -> &str;

    fn on_spawn(&mut self, _ctx: &Context, _entity: &mut TEntity) {}
    fn on_change(&mut self, _ctx: &Context, _entity: &mut TEntity) {}
    fn on_destroy(&mut self, _ctx: &Context, _entity: &mut TEntity) {}
}

#[doc(hidden)]
pub trait AnyAttribute: Send + Sync + std::any::Any {
    fn name(&self) -> &str;

    fn on_spawn(&mut self, _ctx: &Context, _entity: &dyn AnyEntity) {}
    fn on_change(&mut self, _ctx: &Context, _entity: &dyn AnyEntity) {}
    fn on_destroy(&mut self, _ctx: &Context, _entity: &dyn AnyEntity) {}
}

impl std::fmt::Debug for dyn AnyAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.name())
    }
}

impl serde::Serialize for dyn AnyAttribute {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name().serialize(s)
    }
}
