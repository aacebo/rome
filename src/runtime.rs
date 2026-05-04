use crate::{prelude::*, schedule, time};

pub struct Runtime {
    world: Store<World>,
    clock: Box<dyn Clock>,
    layers: Vec<Box<dyn Layer>>,
    scheduler: Box<dyn Scheduler>,
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
            &self.world,
        );

        self.scheduler.on_start(&ctx, &mut self.layers);

        while duration > std::time::Instant::now().duration_since(start) {
            let now = std::time::Instant::now();
            let delta = now - last;
            let tick = self.clock.advance_by(delta);

            ctx = ctx.next(tick);
            last = now;

            for _ in 0..tick.steps {
                self.scheduler.on_tick(&ctx, &mut self.layers);
                self.clock.wait();
            }
        }

        self.scheduler.on_stop(&ctx, &mut self.layers);
    }
}

pub struct RuntimeBuilder {
    clock: Box<dyn Clock>,
    layers: Vec<Box<dyn Layer>>,
    scheduler: Box<dyn Scheduler>,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self {
            clock: Box::new(time::Fixed::new(60)),
            layers: vec![],
            scheduler: Box::new(schedule::Sequence),
        }
    }

    pub fn clock(mut self, clock: impl Clock) -> Self {
        self.clock = Box::new(clock);
        self
    }

    pub fn scheduler(mut self, scheduler: impl Scheduler) -> Self {
        self.scheduler = Box::new(scheduler);
        self
    }

    pub fn layer(mut self, layer: impl Layer) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    pub fn build(self) -> Runtime {
        Runtime {
            world: Store::new(World::new()),
            clock: self.clock,
            layers: self.layers,
            scheduler: self.scheduler,
        }
    }
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
