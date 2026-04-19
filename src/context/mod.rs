mod entity;

pub use entity::*;

use crate::{entity::Entity, world::World};

#[derive(Debug)]
pub struct Context<'a> {
    pub(crate) world: &'a mut World,
}

impl<'a> Context<'a> {
    pub fn with_entity(self, entity: &'a mut Entity) -> EntityContext<'a> {
        EntityContext {
            inner: self,
            entity,
        }
    }
}

impl<'a> std::ops::Deref for Context<'a> {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        self.world
    }
}

impl<'a> std::ops::DerefMut for Context<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.world
    }
}
