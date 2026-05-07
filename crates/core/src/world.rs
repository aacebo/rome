use std::{collections::BTreeMap, sync::Arc};

use crate::{Entity, NodeId, state::State};

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
pub struct WorldId(u64);

impl WorldId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct World {
    id: WorldId,
    node_id: NodeId,
    nodes: BTreeMap<NodeId, State<Arc<dyn Entity>>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn Entity> {
        self.nodes.values().map(|v| v.as_ref().as_ref())
    }

    pub fn exists(&self, id: &NodeId) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn get(&self, id: &NodeId) -> Option<&dyn Entity> {
        match self.nodes.get(id) {
            None => None,
            Some(v) => Some(v.as_ref().as_ref()),
        }
    }

    pub fn spawn<TEntity: Entity>(&mut self, entity: TEntity) -> NodeId {
        let id = self.next_id();
        self.nodes.insert(id, State::new(Arc::new(entity)));
        id
    }

    pub fn destroy(&mut self, id: &NodeId) -> Option<Arc<dyn Entity>> {
        self.nodes.remove(id).map(|v| v.take())
    }

    fn next_id(&mut self) -> NodeId {
        let id = self.node_id;
        self.node_id = self.node_id.next();
        id
    }
}
