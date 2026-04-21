use crate::{Cancellation, Clock, Context, Layer, Scheduler, schedule, time, world::World};

pub struct Runtime {
    world: World,
    clock: Box<dyn Clock>,
    scheduler: Box<dyn Scheduler>,
    layers: Vec<Box<dyn Layer>>,
    cancellation: Option<Cancellation>,
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// Start the runtime which will continue
    /// until cancelled.
    pub fn run(&mut self) {
        let cancellation = Cancellation::default();
        self.cancellation = Some(cancellation.clone());

        let mut last = std::time::Instant::now();
        let mut ctx = Context::new(
            self.clock.advance_by(std::time::Duration::ZERO),
            &mut self.world,
            &cancellation,
        );

        self.scheduler.on_start(&mut ctx, &mut self.layers);

        while !cancellation.is_cancelled() {
            let now = std::time::Instant::now();
            let delta = now - last;
            let tick = self.clock.advance_by(delta);

            ctx = ctx.next(tick);
            last = now;

            for _ in 0..tick.steps {
                self.scheduler.on_tick(&mut ctx, &mut self.layers);

                if cancellation.is_cancelled() {
                    break;
                }

                self.clock.wait();
            }
        }

        self.scheduler.on_stop(&mut ctx, &mut self.layers);
        self.cancellation = None;
    }

    pub fn cancel(&self) {
        if let Some(cancellation) = &self.cancellation {
            cancellation.cancel();
        }
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
            clock: Box::new(time::Fixed::new(60)),
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
            cancellation: None,
        }
    }
}
