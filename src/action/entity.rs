use crate::{
    action::Action,
    diagnostic::Diagnostics,
    entity::{Entity, EntityDraft, EntityId},
    world::World,
};

#[derive(Debug, serde::Serialize)]
#[serde(tag = "name")]
pub enum EntityAction {
    Create { draft: EntityDraft },
    Update { id: EntityId, draft: EntityDraft },
    Delete { id: EntityId },
}

impl Action for EntityAction {
    fn name(&self) -> &str {
        match self {
            Self::Create { draft: _ } => "entity.create",
            Self::Update { id: _, draft: _ } => "entity.update",
            Self::Delete { id: _ } => "entity.delete",
        }
    }

    fn apply(self: Box<Self>, world: &mut World, _diagnostics: &mut Diagnostics) {
        match *self {
            EntityAction::Create { draft } => {
                world.set(Entity {
                    id: world.next_id(),
                    parent_id: draft.parent_id,
                    meta: draft.meta.clone(),
                    name: draft.name,
                    transform: draft.transform,
                    children: draft.children,
                    facets: draft.facets,
                });
            }
            EntityAction::Update { id, draft } => {
                if let Some(entity) = world.get_mut(&id) {
                    entity.parent_id = draft.parent_id;
                    entity.meta = draft.meta;
                    entity.name = draft.name;
                    entity.transform = draft.transform;
                    entity.children = draft.children;
                    entity.facets = draft.facets;
                }
            }
            EntityAction::Delete { id } => world.del(&id),
        }
    }
}
