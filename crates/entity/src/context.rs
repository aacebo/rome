use ayr_diagnostic::{Diagnostic, Diagnostics};
use ayr_state::Store;
use ayr_task::Cancellation;
use ayr_time::Tick;

use crate::world::World;

pub struct Context<'a> {
    tick: Tick,
    store: &'a Store<World>,
    diagnostics: Diagnostics,
    cancellation: &'a Cancellation,
}

impl<'a> Context<'a> {
    pub fn new(tick: Tick, store: &'a Store<World>, cancellation: &'a Cancellation) -> Self {
        Self {
            tick,
            store,
            diagnostics: Diagnostics::new(),
            cancellation,
        }
    }

    pub fn next(mut self, tick: Tick) -> Self {
        self.tick = tick;
        self
    }

    pub fn tick(&self) -> Tick {
        self.tick
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    pub fn cancel(&self) {
        self.cancellation.cancel();
    }

    pub fn emit(&mut self, diagnostic: impl Into<Diagnostic>) -> &mut Self {
        self.diagnostics.write(diagnostic.into());
        self
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.store.flush();
    }
}

impl<'a> std::ops::Deref for Context<'a> {
    type Target = Store<World>;

    fn deref(&self) -> &Self::Target {
        self.store
    }
}
