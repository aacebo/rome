use crate::{
    Cancellation, Tick,
    action::{Action, Actions},
    diagnostic::{Diagnostic, Diagnostics},
    world::World,
};

#[derive(Debug)]
pub struct Context<'a> {
    tick: Tick,
    world: &'a mut World,
    actions: Actions,
    diagnostics: Diagnostics,
    cancellation: &'a Cancellation,
}

impl<'a> Context<'a> {
    pub fn new(tick: Tick, world: &'a mut World, cancellation: &'a Cancellation) -> Self {
        Self {
            tick,
            world,
            actions: Actions::new(),
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

    pub fn world(&self) -> &World {
        self.world
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    pub fn cancel(&self) {
        self.cancellation.cancel();
    }

    pub fn dispatch(&mut self, action: impl Action) -> &mut Self {
        self.actions.write(action);
        self
    }

    pub fn emit(&mut self, diagnostic: impl Into<Diagnostic>) -> &mut Self {
        self.diagnostics.write(diagnostic.into());
        self
    }

    /// Apply buffered actions to a world.
    pub fn apply(&mut self) {
        for action in self.actions.drain() {
            action.apply(self.world, &mut self.diagnostics);
        }
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.apply();
    }
}
