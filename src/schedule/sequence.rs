use crate::{Context, Layer, Scheduler};

pub struct Sequence;

impl Scheduler for Sequence {
    fn on_start(&mut self, ctx: &mut Context, layers: &mut [Box<dyn Layer>]) {
        for layer in layers {
            layer.on_start(ctx);
            ctx.apply();
        }
    }

    fn on_tick(&mut self, ctx: &mut Context, layers: &mut [Box<dyn Layer>]) {
        for layer in layers {
            layer.on_tick(ctx);
            ctx.apply();
        }
    }

    fn on_stop(&mut self, ctx: &mut Context, layers: &mut [Box<dyn Layer>]) {
        for layer in layers {
            layer.on_stop(ctx);
            ctx.apply();
        }
    }
}
