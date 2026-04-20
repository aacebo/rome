use crate::{
    Facet, Layer, Tick,
    action::{Action, ActionBuffer, EntityAction},
    diagnostic::{Diagnostic, DiagnosticId},
    entity::{Entity, EntityDraft, EntityId},
    world::World,
};

pub struct Runtime {
    tick: Tick,
    world: World,
    layers: Vec<Box<dyn Layer>>,
    actions: ActionBuffer,
    diagnostic_id: DiagnosticId,
    diagnostics: Vec<(DiagnosticId, Diagnostic)>,
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    pub fn start(&mut self) {
        for layer in self.layers.iter_mut() {
            let mut state = State {
                world: &self.world,
                actions: &mut self.actions,
            };

            layer.on_start(&mut state);
        }

        self.flush();
    }

    pub fn next(&mut self) {
        self.tick = self.tick.next();

        for layer in self.layers.iter_mut() {
            let mut state = State {
                world: &self.world,
                actions: &mut self.actions,
            };

            layer.on_tick(&mut state);
        }

        self.flush();
    }

    pub fn stop(&mut self) {
        for layer in self.layers.iter_mut() {
            let mut state = State {
                world: &self.world,
                actions: &mut self.actions,
            };

            layer.on_stop(&mut state);
        }

        self.flush();
    }

    pub fn flush(&mut self) {
        while let Some(action) = self.actions.read() {
            match action {
                Action::Emit(diagnostic) => {
                    let id = self.diagnostic_id;
                    self.diagnostic_id = id.next();
                    self.diagnostics.push((id, diagnostic));
                }
                Action::Entity(entity_action) => match entity_action {
                    EntityAction::Create { draft } => {
                        let id = self.world.entity_id;
                        self.world.entity_id = self.world.entity_id.next();
                        let mut facets = draft.facets;
                        let mut entity = Entity {
                            id,
                            parent_id: draft.parent_id,
                            meta: draft.meta,
                            name: draft.name,
                            transform: draft.transform,
                            children: draft.children,
                            facets: vec![],
                        };

                        for facet in facets.iter_mut() {
                            let mut state = State {
                                world: &self.world,
                                actions: &mut self.actions,
                            };

                            facet.on_create(&mut state, &mut entity);
                        }

                        entity.facets = facets;
                        self.world.items.insert(entity.id, entity);
                    }
                    EntityAction::Update { id, draft } => {
                        if let Some(mut entity) = self.world.items.remove(&id) {
                            let mut facets: Vec<Box<dyn Facet>> = entity.facets.drain(..).collect();

                            entity.parent_id = draft.parent_id;
                            entity.meta = draft.meta;
                            entity.name = draft.name;
                            entity.transform = draft.transform;
                            entity.children = draft.children;

                            for facet in facets.iter_mut() {
                                let mut state = State {
                                    world: &self.world,
                                    actions: &mut self.actions,
                                };

                                facet.on_update(&mut state, &mut entity);
                            }

                            entity.facets = facets;
                            self.world.items.insert(entity.id, entity);
                        }
                    }
                    EntityAction::Delete { id } => {
                        if let Some(mut entity) = self.world.items.remove(&id) {
                            let mut facets: Vec<Box<dyn Facet>> = entity.facets.drain(..).collect();

                            for facet in facets.iter_mut() {
                                let mut state = State {
                                    world: &self.world,
                                    actions: &mut self.actions,
                                };

                                facet.on_delete(&mut state, &mut entity);
                            }
                        }
                    }
                },
            }
        }
    }
}

pub struct RuntimeBuilder {
    layers: Vec<Box<dyn Layer>>,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn layer(mut self, layer: impl Layer) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    pub fn build(self) -> Runtime {
        Runtime {
            tick: Tick::default(),
            world: World::new(),
            layers: self.layers,
            actions: ActionBuffer::new(),
            diagnostic_id: DiagnosticId::default(),
            diagnostics: vec![],
        }
    }
}

pub struct State<'a> {
    world: &'a World,
    actions: &'a mut ActionBuffer,
}

impl<'a> State<'a> {
    pub fn is_empty(&self) -> bool {
        self.world.is_empty()
    }

    pub fn len(&self) -> usize {
        self.world.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.world.iter()
    }

    pub fn has(&self, id: &EntityId) -> bool {
        self.world.has(id)
    }

    pub fn get(&self, id: &EntityId) -> Option<&Entity> {
        self.world.get(id)
    }

    pub fn create(&mut self, draft: EntityDraft) {
        self.actions
            .write(Action::Entity(EntityAction::Create { draft }));
    }

    pub fn update(&mut self, id: EntityId, draft: EntityDraft) {
        self.actions
            .write(Action::Entity(EntityAction::Update { id, draft }));
    }

    pub fn delete(&mut self, id: EntityId) {
        self.actions
            .write(Action::Entity(EntityAction::Delete { id }));
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) {
        self.actions.write(Action::from(diagnostic));
    }
}
