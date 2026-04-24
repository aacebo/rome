mod entity;

pub use entity::*;

use crate::ContextMut;

/// An Action represents a request for state to be changed.
pub trait Action: std::fmt::Debug + 'static {
    /// ex. `entity.create`
    fn name(&self) -> &str;

    /// Called by the Runtime to persist an Action
    /// to the State.
    fn apply(self: Box<Self>, ctx: &mut ContextMut);
}

#[derive(Debug, Default)]
pub struct Actions(Vec<Box<dyn Action>>);

impl Actions {
    pub fn new() -> Self {
        Self::default()
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

    pub fn drain(&mut self) -> std::vec::Drain<'_, Box<dyn Action>> {
        self.0.drain(..)
    }

    pub fn write(&mut self, action: impl Action) -> &mut Self {
        self.0.push(Box::new(action));
        self
    }
}
