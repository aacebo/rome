use std::collections::{BTreeMap, VecDeque};

use crate::{
    Facet, Tick,
    context::Context,
    diagnostic::{Diagnostic, DiagnosticBuilder, DiagnosticId},
    entity::{Entity, EntityDraft, EntityId},
    math,
    meta::Meta,
};

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

    pub fn create(
        &mut self,
        name: impl Into<String>,
        transform: math::Transform,
        fnc: impl FnOnce(&mut EntityDraft),
    ) {
        let id = self.entity_id;
        self.entity_id = self.entity_id.next();
        let mut draft = EntityDraft {
            parent_id: None,
            meta: Meta::default(),
            name: name.into(),
            transform,
            children: vec![],
            facets: vec![],
        };

        fnc(&mut draft);

        let mut entity = Entity {
            id,
            parent_id: draft.parent_id,
            meta: draft.meta,
            name: draft.name,
            transform: draft.transform,
            children: draft.children,
            facets: vec![],
        };

        let mut ctx = Context { world: self }.with_entity(&mut entity);

        for facet in draft.facets.iter_mut() {
            facet.on_create(&mut ctx);
        }

        entity.facets = draft.facets;
        self.items.insert(id, entity);
    }

    pub fn update(&mut self, id: &EntityId, fnc: impl FnOnce(&mut Entity)) {
        if let Some(entity) = self.get_mut(id) {
            fnc(entity);
        }
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

    pub fn emit(&mut self, fnc: impl FnOnce(DiagnosticBuilder) -> Diagnostic) {
        let id = self.diagnostic_id;
        self.diagnostic_id = self.diagnostic_id.next();
        self.diagnostics.push_back(fnc(DiagnosticBuilder::new(id)));
    }

    pub fn next(&mut self) {
        self.tick = self.tick.next();
    }
}
