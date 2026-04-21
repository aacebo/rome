use crate::{Clock, Context, Layer, Scheduler, schedule, time, world::World};

pub struct Runtime {
    world: World,
    clock: Box<dyn Clock>,
    scheduler: Box<dyn Scheduler>,
    layers: Vec<Box<dyn Layer>>,
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    /// Start the runtime which will continue
    /// until a Stop action is received.
    pub fn next(&mut self, delta: std::time::Duration) {
        let tick = self.clock.advance_by(delta);
        let mut ctx = Context::new(tick, &mut self.world);

        self.scheduler.on_start(&mut ctx, &mut self.layers);
        self.scheduler.on_tick(&mut ctx, &mut self.layers);
        self.scheduler.on_stop(&mut ctx, &mut self.layers);
    }
}

pub struct RuntimeBuilder {
    clock: Box<dyn Clock>,
    scheduler: Box<dyn Scheduler>,
    layers: Vec<Box<dyn Layer>>,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self {
            clock: Box::new(time::Fixed::from_hz(60)),
            scheduler: Box::new(schedule::Sequence),
            layers: vec![],
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
            world: World::new(),
            clock: self.clock,
            scheduler: self.scheduler,
            layers: self.layers,
        }
    }
}
