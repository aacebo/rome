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
        let actions: Vec<Box<dyn Action>> = self.actions.drain().collect();
        let mut ctx = self.mutable();

        for action in actions {
            action.apply(&mut ctx);
        }
    }

    pub fn mutable(&mut self) -> ContextMut<'_, 'a> {
        ContextMut(self)
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.apply();
    }
}

impl<'a> std::ops::Deref for Context<'a> {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        self.world
    }
}

/// A mutable view of a [`Context`] handed to [`Action::apply`].
///
/// Derefs to [`World`] for direct mutating access. Context-level operations
/// (`dispatch`, `emit`, `tick`, `cancel`, etc.) are exposed as inherent methods.
#[derive(Debug)]
pub struct ContextMut<'a, 'b>(&'a mut Context<'b>);

impl<'a, 'b> ContextMut<'a, 'b> {
    pub fn context(&mut self) -> &mut Context<'b> {
        self.0
    }

    pub fn tick(&self) -> Tick {
        self.0.tick
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.is_cancelled()
    }

    pub fn cancel(&self) {
        self.0.cancel();
    }

    pub fn dispatch(&mut self, action: impl Action) -> &mut Self {
        self.0.dispatch(action);
        self
    }

    pub fn emit(&mut self, diagnostic: impl Into<Diagnostic>) -> &mut Self {
        self.0.emit(diagnostic);
        self
    }
}

impl<'a, 'b> std::ops::Deref for ContextMut<'a, 'b> {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        self.0.world
    }
}

impl<'a, 'b> std::ops::DerefMut for ContextMut<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.world
    }
}
