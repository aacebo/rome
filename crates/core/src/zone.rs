use super::Context;

pub trait Zone: Send + Sync + std::fmt::Debug + std::any::Any {
    fn name(&self) -> &'static str;

    /// Called when the scene is entered/started.
    fn on_enter(&mut self, _ctx: &Context) {}

    /// Called before rendering.
    fn on_tick(&mut self, _ctx: &Context) {}

    /// Called when the scene is exited/stopped.
    fn on_exit(&mut self, _ctx: &Context) {}
}
