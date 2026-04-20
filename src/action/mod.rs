mod entity;
mod system;

pub use entity::*;
pub use system::*;

use crate::{
    diagnostic::{Diagnostic, DiagnosticBuffer},
    world::World,
};

/// An Action represents a request for state to be changed.
pub trait Action: std::fmt::Debug + 'static {
    /// ex. `entity.create`
    fn name(&self) -> &str;

    /// Called by the Runtime to persist an Action
    /// to the State.
    fn apply(self: Box<Self>, world: &mut World, diagnostics: &mut DiagnosticBuffer);
}

impl Action for Diagnostic {
    fn name(&self) -> &str {
        "diagnostic"
    }

    fn apply(self: Box<Self>, _world: &mut World, diagnostics: &mut DiagnosticBuffer) {
        diagnostics.write(*self);
    }
}

#[derive(Debug)]
pub struct ActionBuffer(Vec<Box<dyn Action>>);

impl ActionBuffer {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn first(&self) -> Option<&dyn Action> {
        match self.0.first() {
            None => None,
            Some(v) => Some(v.as_ref()),
        }
    }

    pub fn last(&self) -> Option<&dyn Action> {
        match self.0.last() {
            None => None,
            Some(v) => Some(v.as_ref()),
        }
    }

    pub fn read(&mut self) -> Option<Box<dyn Action>> {
        self.0.pop()
    }

    pub fn write(&mut self, action: impl Action) -> &mut Self {
        self.0.push(Box::new(action));
        self
    }
}
