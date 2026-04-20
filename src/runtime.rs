use crate::{
    Facet, Tick,
    action::{Action, ActionBuffer, EntityAction},
    diagnostic::{Diagnostic, DiagnosticId},
    entity::{Entity, EntityDraft, EntityId},
    world::World,
};

pub struct Runtime {
    tick: Tick,
    world: World,
    history: Vec<(Tick, World)>,
    actions: ActionBuffer,
    diagnostic_id: DiagnosticId,
    diagnostics: Vec<(DiagnosticId, Diagnostic)>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            tick: Tick::default(),
            world: World::new(),
            history: vec![],
            actions: ActionBuffer::new(),
            diagnostic_id: DiagnosticId::default(),
            diagnostics: vec![],
        }
    }

    pub fn state(&mut self) -> State<'_> {
        State {
            world: &self.world,
            actions: &mut self.actions,
        }
    }

    pub fn next(mut self) {
        let tick = self.tick.next();
        let mut state = State {
            world: &mut self.world,
            actions: &mut self.actions,
        };

        self.history.push((self.tick, self.world));
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
                            let mut state = EntityState {
                                world: &self.world,
                                entity: &mut entity,
                                actions: &mut self.actions,
                            };

                            facet.on_create(&mut state);
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
                                let mut state = EntityState {
                                    world: &self.world,
                                    entity: &mut entity,
                                    actions: &mut self.actions,
                                };

                                facet.on_update(&mut state);
                            }

                            entity.facets = facets;
                            self.world.items.insert(entity.id, entity);
                        }
                    }
                    EntityAction::Delete { id } => {
                        if let Some(mut entity) = self.world.items.remove(&id) {
                            let mut facets: Vec<Box<dyn Facet>> = entity.facets.drain(..).collect();

                            for facet in facets.iter_mut() {
                                let mut state = EntityState {
                                    world: &self.world,
                                    entity: &mut entity,
                                    actions: &mut self.actions,
                                };

                                facet.on_delete(&mut state);
                            }
                        }
                    }
                },
            }
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

pub struct EntityState<'a> {
    world: &'a World,
    entity: &'a mut Entity,
    actions: &'a mut ActionBuffer,
}

impl<'a> EntityState<'a> {
    pub fn entity(&self) -> &Entity {
        self.entity
    }

    pub fn entity_mut(&mut self) -> &mut Entity {
        self.entity
    }

    pub fn parent(&self) -> Option<&Entity> {
        match &self.entity.parent_id {
            None => None,
            Some(id) => self.world.get(id),
        }
    }

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
