use crate::{Context, Layer, Scheduler, Tick, schedule, world::World};

pub struct Runtime {
    tick: Tick,
    world: World,
    scheduler: Box<dyn Scheduler>,
    layers: Vec<Box<dyn Layer>>,
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    /// Start the runtime which will continue
    /// until a Stop action is received.
    pub fn start(&mut self) {
        self.tick = self.tick.next();
        let mut ctx = Context::new(self.tick, &mut self.world);
        self.scheduler.on_start(&mut ctx, &mut self.layers);

        while !ctx.is_cancelled() {
            self.scheduler.on_tick(&mut ctx, &mut self.layers);
        }

        self.scheduler.on_stop(&mut ctx, &mut self.layers);
    }
}

pub struct RuntimeBuilder {
    scheduler: Box<dyn Scheduler>,
    layers: Vec<Box<dyn Layer>>,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self {
            scheduler: Box::new(schedule::Sequence),
            layers: vec![],
        }
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
            tick: Tick::default(),
            world: World::new(),
            scheduler: self.scheduler,
            layers: self.layers,
        }
    }
}
