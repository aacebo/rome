use super::Scheduler;
use crate::prelude::{Context, Scene};

pub struct Sequence;

impl Scheduler for Sequence {
    fn on_start(&mut self, ctx: &Context, _scenes: &mut [Box<dyn Scene>]) {
        for layer in _scenes {
            layer.on_enter(ctx);
            ctx.flush();
        }
    }

    fn on_tick(&mut self, ctx: &Context, _scenes: &mut [Box<dyn Scene>]) {
        for layer in _scenes.iter_mut() {
            layer.on_render(ctx);
            ctx.flush();
        }
    }

    fn on_stop(&mut self, ctx: &Context, _scenes: &mut [Box<dyn Scene>]) {
        for layer in _scenes {
            layer.on_exit(ctx);
            ctx.flush();
        }
    }
}
