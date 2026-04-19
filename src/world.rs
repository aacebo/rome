use std::collections::{BTreeMap, VecDeque};

use crate::{
    diagnostic::{Diagnostic, DiagnosticBuilder, DiagnosticId},
    entity::{Entity, EntityId},
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

#[derive(Debug, Default, Clone, serde::Serialize)]
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

    pub fn put(&mut self, entity: Entity) {
        // let id = entity.id;
        // let exists = self.has(&id);
        // let mut ctx = RefCell::new(Context { world: self }.with_entity(&mut entity));

        // if exists {
        //     let reference = ctx.get_mut();

        //     for facet in reference.entity_mut().facets.iter_mut() {
        //         if let Some(v) = Arc::get_mut(facet) {
        //             v.on_update(reference);
        //         }
        //     }
        // } else {

        // }

        self.items.insert(entity.id, entity);
    }

    pub fn delete(&mut self, id: &EntityId) {
        self.items.remove(id);
    }

    pub fn create(&mut self, name: impl Into<String>, transform: math::Transform) -> &mut Entity {
        let id = self.entity_id;
        self.entity_id = self.entity_id.next();
        self.items.entry(id).or_insert(Entity {
            id,
            parent_id: None,
            meta: Meta::default(),
            name: name.into(),
            transform,
            children: vec![],
            facets: vec![],
        })
    }

    pub fn emit(&mut self, func: impl FnOnce(DiagnosticBuilder) -> Diagnostic) {
        let id = self.diagnostic_id;
        self.diagnostic_id = self.diagnostic_id.next();
        self.diagnostics.push_back(func(DiagnosticBuilder::new(id)));
    }
}
