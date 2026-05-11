use std::rc::Rc;

use crate::{Attribute, Entity};

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
pub struct NodeId(u64);

impl NodeId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Node {
    Entity {
        parent: Option<NodeId>,
        entity: Rc<dyn Entity>,
        children: Vec<NodeId>,
        attributes: Vec<Rc<dyn Attribute>>,
    },
}
