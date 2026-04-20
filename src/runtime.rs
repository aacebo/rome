use crate::{
    Layer, Tick,
    action::{ActionBuffer, EntityAction, SystemAction},
    diagnostic::{Diagnostic, DiagnosticBuffer},
    entity::{Entity, EntityDraft, EntityId},
    world::World,
};

pub struct Runtime {
    tick: Tick,
    world: World,
    layers: Vec<Box<dyn Layer>>,
    actions: ActionBuffer,
    diagnostics: DiagnosticBuffer,
}

impl Runtime {
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    /// Start the runtime which will continue
    /// until a Stop action is received.
    pub fn start(&mut self) {
        for layer in self.layers.iter_mut() {
            let mut state = State {
                world: &self.world,
                actions: &mut self.actions,
            };

            layer.on_start(&mut state);
        }

        self.flush();

        loop {
            self.next();

            if let Some(SystemAction::Stop) = self.flush() {
                break;
            };
        }

        self.flush();

        for layer in self.layers.iter_mut() {
            let mut state = State {
                world: &self.world,
                actions: &mut self.actions,
            };

            layer.on_stop(&mut state);
        }

        self.flush();
    }

    /// Issue a Stop action.
    pub fn stop(&mut self) {
        self.actions.write(SystemAction::Stop);
    }

    fn next(&mut self) {
        self.tick = self.tick.next();

        for layer in self.layers.iter_mut() {
            let mut state = State {
                world: &self.world,
                actions: &mut self.actions,
            };

            layer.on_tick(&mut state);
        }
    }

    fn flush(&mut self) -> Option<SystemAction> {
        while let Some(action) = self.actions.read() {
            action.apply(&mut self.world, &mut self.diagnostics);
        }

        None
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
            diagnostics: DiagnosticBuffer::new(),
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
        self.actions.write(EntityAction::Create { draft });
    }

    pub fn update(&mut self, id: EntityId, draft: EntityDraft) {
        self.actions.write(EntityAction::Update { id, draft });
    }

    pub fn delete(&mut self, id: EntityId) {
        self.actions.write(EntityAction::Delete { id });
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) {
        self.actions.write(diagnostic);
    }
}
