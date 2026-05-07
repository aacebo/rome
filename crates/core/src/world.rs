use std::collections::BTreeMap;

use crate::{Node, NodeId, state::State};

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
    nodes: BTreeMap<NodeId, State<Node>>,
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

    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values().map(|v| v.as_ref())
    }

    pub fn exists(&self, id: &NodeId) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn get(&self, id: &NodeId) -> Option<&Node> {
        match self.nodes.get(id) {
            None => None,
            Some(v) => Some(v.as_ref()),
        }
    }

    pub fn get_mut(&mut self, id: &NodeId) -> Option<&mut Node> {
        match self.nodes.get_mut(id) {
            None => None,
            Some(v) => Some(v.as_mut()),
        }
    }

    pub fn spawn(&mut self, mut node: Node) {
        node.id = self.next_id();
        self.nodes.insert(node.id, node.into());
    }

    pub fn destroy(&mut self, id: &NodeId) -> Option<Node> {
        self.nodes.remove(id).map(|v| v.take())
    }

    fn next_id(&mut self) -> NodeId {
        let id = self.node_id;
        self.node_id = self.node_id.next();
        id
    }
}
