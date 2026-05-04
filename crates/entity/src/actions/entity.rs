use std::sync::Arc;

use crate::{Entity, EntityId, Facet, Meta, World};

#[derive(Debug, serde::Serialize)]
pub struct SpawnEntity {
    pub parent_id: Option<EntityId>,
    pub meta: Meta,
    pub name: String,
    pub transform: ayr_math::Transform,
    pub facets: Vec<Arc<dyn Facet>>,
    pub children: Vec<EntityId>,
}

impl ayr_state::Action for SpawnEntity {
    type State = World;

    fn name(&self) -> &'static str {
        "entity.spawn"
    }

    fn reduce(&self, world: &mut Self::State) {
        let id = world.next_id();

        world.set(Entity {
            id,
            parent_id: self.parent_id,
            meta: self.meta.clone(),
            name: self.name.clone(),
            transform: self.transform,
            facets: self.facets.clone(),
            children: self.children.clone(),
        });
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DestroyEntity {
    pub id: EntityId,
}

impl ayr_state::Action for DestroyEntity {
    type State = World;

    fn name(&self) -> &'static str {
        "entity.destroy"
    }

    fn reduce(&self, world: &mut Self::State) {
        world.del(&self.id);
    }
}
