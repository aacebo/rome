use std::collections::{BTreeMap, VecDeque};

use crate::{
    context::Context,
    diagnostic::{Diagnostic, DiagnosticBuilder, DiagnosticId},
    entity::{Entity, EntityId, Facet},
    math,
    meta::Meta,
};

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
pub struct Tick(u64);

impl Tick {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct World {
    tick: Tick,
    entity_id: EntityId,
    diagnostic_id: DiagnosticId,
    items: BTreeMap<EntityId, Entity>,
    diagnostics: VecDeque<Diagnostic>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
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

    pub fn put(&mut self, mut entity: Entity) {
        let is_new = !self.items.contains_key(&entity.id);
        let mut facets: Vec<Box<dyn Facet>> = entity.facets.drain(..).collect();
        let ctx = Context { world: self };
        let mut entity_ctx = ctx.with_entity(&mut entity);

        for facet in facets.iter_mut() {
            if is_new {
                facet.on_create(&mut entity_ctx);
            } else {
                facet.on_update(&mut entity_ctx);
            }
        }

        entity.facets = facets;
        self.items.insert(entity.id, entity);
    }

    pub fn delete(&mut self, id: &EntityId) {
        if let Some(mut entity) = self.items.remove(id) {
            let mut facets: Vec<Box<dyn Facet>> = entity.facets.drain(..).collect();
            let ctx = Context { world: self };
            let mut entity_ctx = ctx.with_entity(&mut entity);

            for facet in facets.iter_mut() {
                facet.on_delete(&mut entity_ctx);
            }
        }
    }

    pub fn create(&mut self, name: impl Into<String>, transform: math::Transform) -> &mut Entity {
        let id = self.entity_id;
        self.entity_id = self.entity_id.next();

        let mut entity = Entity {
            id,
            parent_id: None,
            meta: Meta::default(),
            name: name.into(),
            transform,
            children: vec![],
            facets: vec![],
        };

        let mut facets: Vec<Box<dyn Facet>> = entity.facets.drain(..).collect();
        let ctx = Context { world: self };
        let mut entity_ctx = ctx.with_entity(&mut entity);

        for facet in facets.iter_mut() {
            facet.on_create(&mut entity_ctx);
        }

        entity.facets = facets;
        self.items.entry(id).or_insert(entity)
    }

    pub fn emit(&mut self, func: impl FnOnce(DiagnosticBuilder) -> Diagnostic) {
        let id = self.diagnostic_id;
        self.diagnostic_id = self.diagnostic_id.next();
        self.diagnostics.push_back(func(DiagnosticBuilder::new(id)));
    }
}
