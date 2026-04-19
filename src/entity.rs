use crate::{context::EntityContext, math, meta::Meta};

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

impl Entity {
    pub fn access(&mut self) -> Accessor<'_> {
        Accessor::from(self)
    }
}

pub struct Accessor<'a> {
    entity: &'a mut Entity,
}

impl<'a> Accessor<'a> {
    pub fn parent(self, id: EntityId) -> Self {
        self.entity.parent_id = Some(id);
        self
    }

    pub fn meta(self, meta: Meta) -> Self {
        self.entity.meta = meta;
        self
    }

    pub fn child(self, id: EntityId) -> Self {
        self.entity.children.push(id);
        self
    }

    pub fn facet(self, facet: impl Facet) -> Self {
        self.entity.facets.push(Box::new(facet));
        self
    }

    pub fn save(self) -> &'a mut Entity {
        self.entity
    }
}

impl<'a> From<&'a mut Entity> for Accessor<'a> {
    fn from(entity: &'a mut Entity) -> Self {
        Self { entity }
    }
}

/// A Facet represents a single, focused aspect of an Entity's state and behavior.
///
/// Facets are the primary building blocks used to compose Entities. Each Facet
/// should encapsulate one concern (e.g. health, rendering, movement) and avoid
/// depending directly on other Facets.
///
/// Complex interactions between Facets should be coordinated externally
/// (e.g. via systems or commands) rather than through tight coupling.
///
/// In this model:
/// - Entities provide identity and composition
/// - Facets provide capabilities
pub trait Facet: Send + Sync + 'static {
    fn name(&self) -> &str;

    fn on_create(&mut self, _ctx: &mut EntityContext) {}
    fn on_update(&mut self, _ctx: &mut EntityContext) {}
    fn on_delete(&mut self, _ctx: &mut EntityContext) {}
}

impl serde::Serialize for dyn Facet {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name().serialize(s)
    }
}

impl std::fmt::Debug for dyn Facet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.name())
    }
}
