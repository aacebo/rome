use crate::{
    ContextMut,
    action::Action,
    entity::{Entity, EntityDraft, EntityId},
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

    fn apply(self: Box<Self>, ctx: &mut ContextMut) {
        match *self {
            EntityAction::Create { draft } => {
                let id = ctx.next_id();
                let mut entity = Entity {
                    id,
                    parent_id: draft.parent_id,
                    meta: draft.meta.clone(),
                    name: draft.name,
                    transform: draft.transform,
                    children: draft.children,
                    facets: draft.facets,
                };

                let mut facets = std::mem::take(&mut entity.facets);

                for facet in facets.iter_mut() {
                    facet.on_create(ctx.context(), &mut entity);
                }

                entity.facets = facets;
                ctx.set(entity);
            }
            EntityAction::Update { id, draft } => {
                let Some(mut entity) = ctx.take(&id) else {
                    return;
                };

                entity.parent_id = draft.parent_id;
                entity.meta = draft.meta;
                entity.name = draft.name;
                entity.transform = draft.transform;
                entity.children = draft.children;
                entity.facets = draft.facets;

                let mut facets = std::mem::take(&mut entity.facets);

                for facet in facets.iter_mut() {
                    facet.on_update(ctx.context(), &mut entity);
                }

                entity.facets = facets;
                ctx.set(entity);
            }
            EntityAction::Delete { id } => {
                let Some(mut entity) = ctx.take(&id) else {
                    return;
                };

                let mut facets = std::mem::take(&mut entity.facets);

                for facet in facets.iter_mut() {
                    facet.on_delete(ctx.context(), &mut entity);
                }
            }
        }
    }
}
