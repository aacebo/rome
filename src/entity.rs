use crate::{Facet, math, meta::Meta};

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
pub struct EntityId(u64);

impl EntityId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

/// An Entity is a world object with identity that is composed from one or more Facets.
///
/// Entities represent the primary objects that exist within the simulation or scene.
/// On their own, Entities are typically lightweight and act as containers for
/// state and capabilities defined by attached Facets.
///
/// In this model:
/// - Entities provide identity and composition
/// - Facets provide focused capabilities, state, and optional behavior
///
/// Complex behavior emerges from the combination of Facets attached to an Entity.
#[derive(Debug, serde::Serialize)]
pub struct Entity {
    pub id: EntityId,
    pub parent_id: Option<EntityId>,
    pub meta: Meta,
    pub name: String,
    pub transform: math::Transform,
    pub children: Vec<EntityId>,
    pub facets: Vec<Box<dyn Facet>>,
}

#[derive(Debug, serde::Serialize)]
pub struct EntityDraft {
    pub parent_id: Option<EntityId>,
    pub meta: Meta,
    pub name: String,
    pub transform: math::Transform,
    pub children: Vec<EntityId>,
    pub facets: Vec<Box<dyn Facet>>,
}
