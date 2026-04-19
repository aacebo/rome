use crate::entity::Entity;

#[derive(Debug)]
pub struct EntityContext<'a> {
    pub(super) inner: super::Context<'a>,
    pub(super) entity: &'a mut Entity,
}

impl<'a> EntityContext<'a> {
    pub fn parent(&self) -> Option<&Entity> {
        match &self.entity.parent_id {
            None => None,
            Some(id) => self.inner.world.get(id),
        }
    }

    pub fn parent_mut(&mut self) -> Option<&mut Entity> {
        match &self.entity.parent_id {
            None => None,
            Some(id) => self.inner.world.get_mut(id),
        }
    }

    pub fn entity(&self) -> &Entity {
        self.entity
    }

    pub fn entity_mut(&mut self) -> &mut Entity {
        self.entity
    }
}

impl<'a> std::ops::Deref for EntityContext<'a> {
    type Target = super::Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> std::ops::DerefMut for EntityContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
