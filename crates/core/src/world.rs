use std::collections::BTreeMap;

use crate::{Node, NodeId};

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
    items: BTreeMap<NodeId, Node>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.items.values()
    }

    pub fn has(&self, id: &NodeId) -> bool {
        self.items.contains_key(id)
    }

    pub fn get(&self, id: &NodeId) -> Option<&Node> {
        self.items.get(id)
    }

    pub fn get_mut(&mut self, id: &NodeId) -> Option<&mut Node> {
        self.items.get_mut(id)
    }

    pub fn set(&mut self, node: Node) {
        self.items.insert(node.id, node);
    }

    pub fn del(&mut self, id: &NodeId) {
        self.items.remove(id);
    }

    pub fn take(&mut self, id: &NodeId) -> Option<Node> {
        self.items.remove(id)
    }

    pub fn next_id(&mut self) -> NodeId {
        let id = self.node_id;
        self.node_id = self.node_id.next();
        id
    }
}
