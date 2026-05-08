use super::Context;

pub trait Layer: Send + Sync + std::any::Any {
    fn name(&self) -> &'static str;

    fn on_start(&mut self, _ctx: &Context) {}
    fn on_tick(&mut self, _ctx: &Context) {}
    fn on_stop(&mut self, _ctx: &Context) {}
}
