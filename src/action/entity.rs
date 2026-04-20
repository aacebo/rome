use crate::entity::{EntityDraft, EntityId};

#[derive(Debug, serde::Serialize)]
#[serde(tag = "name")]
pub enum EntityAction {
    Create { draft: EntityDraft },
    Update { id: EntityId, draft: EntityDraft },
    Delete { id: EntityId },
}
