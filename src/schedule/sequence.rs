use super::Scheduler;
use crate::prelude::{Context, Scene};

pub struct Sequence;

impl Scheduler for Sequence {
    fn on_start(&mut self, ctx: &mut Context, scenes: &mut [Box<dyn Scene>]) {
        for scene in scenes {
            scene.on_enter(ctx);
            ctx.flush();
        }
    }

    fn on_tick(&mut self, ctx: &mut Context, scenes: &mut [Box<dyn Scene>]) {
        for scene in scenes.iter_mut() {
            scene.on_tick(ctx);
            ctx.flush();
        }
    }

    fn on_stop(&mut self, ctx: &mut Context, scenes: &mut [Box<dyn Scene>]) {
        for scene in scenes {
            scene.on_exit(ctx);
            ctx.flush();
        }
    }
}
