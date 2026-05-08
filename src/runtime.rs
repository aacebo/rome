use crate::{prelude::*, state::Store, time};

pub struct Runtime {
    world: Store<World>,
    clock: Box<dyn Clock>,
    layers: Vec<Box<dyn Layer>>,
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    pub fn world(&self) -> &Store<World> {
        &self.world
    }

    pub fn run(&mut self, duration: std::time::Duration) {
        let start = std::time::Instant::now();
        let mut last = std::time::Instant::now();
        let mut ctx = Context::new(
            self.clock.advance_by(std::time::Duration::ZERO),
            &mut self.world,
        );

        for layer in self.layers.iter_mut() {
            layer.on_start(&ctx);
            ctx.flush();
        }

        while duration > std::time::Instant::now().duration_since(start) {
            let now = std::time::Instant::now();
            let delta = now - last;
            let tick = self.clock.advance_by(delta);

            ctx = ctx.next(tick);
            last = now;

            for _ in 0..tick.steps {
                for layer in self.layers.iter_mut() {
                    layer.on_tick(&ctx);
                    ctx.flush();
                }

                self.clock.wait();
            }
        }

        for layer in self.layers.iter_mut() {
            layer.on_stop(&ctx);
            ctx.flush();
        }
    }
}

pub struct RuntimeBuilder {
    clock: Box<dyn Clock>,
    layers: Vec<Box<dyn Layer>>,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self {
            clock: Box::new(time::Fixed::new(60)),
            layers: vec![],
        }
    }

    pub fn clock(mut self, clock: impl Clock) -> Self {
        self.clock = Box::new(clock);
        self
    }

    pub fn layer(mut self, scene: impl Layer) -> Self {
        self.layers.push(Box::new(scene));
        self
    }

    pub fn build(self) -> Runtime {
        Runtime {
            world: Store::new(World::new()),
            clock: self.clock,
            layers: self.layers,
        }
    }
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
