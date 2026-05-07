use super::Scheduler;
use crate::prelude::{Context, Layer};

pub struct Sequence;

impl Scheduler for Sequence {
    fn on_start(&mut self, ctx: &mut Context, layers: &mut [Box<dyn Layer>]) {
        for scene in layers {
            scene.on_enter(ctx);
            ctx.flush();
        }
    }

    fn on_tick(&mut self, ctx: &mut Context, layers: &mut [Box<dyn Layer>]) {
        for scene in layers.iter_mut() {
            scene.on_tick(ctx);
            ctx.flush();
        }
    }

    fn on_stop(&mut self, ctx: &mut Context, layers: &mut [Box<dyn Layer>]) {
        for scene in layers {
            scene.on_exit(ctx);
            ctx.flush();
        }
    }
}
