use crate::{
    CancelSource, CancelToken, Tick,
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
    cancel_source: CancelSource,
    cancel_token: CancelToken,
}

impl<'a> Context<'a> {
    pub fn new(tick: Tick, world: &'a mut World) -> Self {
        let cancel_source = CancelSource::new();
        let cancel_token = cancel_source.token();

        Self {
            tick,
            world,
            actions: Actions::new(),
            diagnostics: Diagnostics::new(),
            cancel_source,
            cancel_token,
        }
    }

    pub fn tick(&self) -> Tick {
        self.tick
    }

    pub fn world(&self) -> &World {
        self.world
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel_token.is_cancelled()
    }

    pub fn cancel(&self) {
        self.cancel_source.cancel();
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
