mod entity;

pub use entity::*;

use crate::diagnostic::Diagnostic;

#[derive(Debug, serde::Serialize)]
#[serde(tag = "type")]
pub enum Action {
    Emit(Diagnostic),
    Entity(EntityAction),
}

impl From<Diagnostic> for Action {
    fn from(value: Diagnostic) -> Self {
        Self::Emit(value)
    }
}

impl From<EntityAction> for Action {
    fn from(value: EntityAction) -> Self {
        Self::Entity(value)
    }
}

#[derive(Debug)]
pub struct ActionBuffer(Vec<Action>);

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

    pub fn first(&self) -> Option<&Action> {
        self.0.first()
    }

    pub fn last(&self) -> Option<&Action> {
        self.0.last()
    }

    pub fn read(&mut self) -> Option<Action> {
        self.0.pop()
    }

    pub fn write(&mut self, action: Action) -> &mut Self {
        self.0.push(action);
        self
    }
}
