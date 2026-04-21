use std::collections::BTreeMap;

use crate::entity::{Entity, EntityId};

#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(transparent)]
pub struct WorldId(u64);

impl WorldId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct World {
    id: WorldId,
    entity_id: EntityId,
    items: BTreeMap<EntityId, Entity>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.items.values()
    }

    pub fn has(&self, id: &EntityId) -> bool {
        self.items.contains_key(id)
    }

    pub fn get(&self, id: &EntityId) -> Option<&Entity> {
        self.items.get(id)
    }

    pub fn get_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.items.get_mut(id)
    }

    pub fn set(&mut self, entity: Entity) {
        self.items.insert(entity.id, entity);
    }

    pub fn del(&mut self, id: &EntityId) {
        self.items.remove(id);
    }

    pub fn take(&mut self, id: &EntityId) -> Option<Entity> {
        self.items.remove(id)
    }

    pub fn next_id(&mut self) -> EntityId {
        let id = self.entity_id;
        self.entity_id = self.entity_id.next();
        id
    }
}
