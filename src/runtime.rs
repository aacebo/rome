use crate::{Context, Layer, Tick, world::World};

pub struct Runtime {
    tick: Tick,
    world: World,
    layers: Vec<Box<dyn Layer>>,
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    /// Start the runtime which will continue
    /// until a Stop action is received.
    pub fn start(&mut self) {
        let mut ctx = Context::new(&mut self.world);

        for layer in self.layers.iter_mut() {
            layer.on_start(&mut ctx);
            ctx.apply();
        }

        while !ctx.is_cancelled() {
            self.tick = self.tick.next();

            for layer in self.layers.iter_mut() {
                layer.on_tick(&mut ctx);
                ctx.apply();
            }
        }

        for layer in self.layers.iter_mut() {
            layer.on_stop(&mut ctx);
            ctx.apply();
        }
    }
}

pub struct RuntimeBuilder {
    layers: Vec<Box<dyn Layer>>,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn layer(mut self, layer: impl Layer) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    pub fn build(self) -> Runtime {
        Runtime {
            tick: Tick::default(),
            world: World::new(),
            layers: self.layers,
        }
    }
}
