use ayr_diagnostic::Diagnostic;
use ayr_state::Store;
use ayr_time::Tick;

use crate::world::World;

pub struct Context<'a> {
    tick: Tick,
    world: &'a Store<World>,
    diagnostics: crossbeam::queue::SegQueue<Diagnostic>,
}

impl<'a> Context<'a> {
    pub fn new(tick: Tick, world: &'a Store<World>) -> Self {
        Self {
            tick,
            world,
            diagnostics: crossbeam::queue::SegQueue::new(),
        }
    }

    pub fn next(mut self, tick: Tick) -> Self {
        self.tick = tick;
        self
    }

    pub fn tick(&self) -> Tick {
        self.tick
    }

    pub fn emit(&self, diagnostic: impl Into<Diagnostic>) -> &Self {
        self.diagnostics.push(diagnostic.into());
        self
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.world.flush();

        while let Some(diagnostic) = self.diagnostics.pop() {
            println!("{}", diagnostic);
        }
    }
}

impl<'a> std::ops::Deref for Context<'a> {
    type Target = Store<World>;

    fn deref(&self) -> &Self::Target {
        self.world
    }
}
