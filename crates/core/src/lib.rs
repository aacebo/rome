pub mod context;
pub mod prelude;
pub mod state;
pub mod view;
pub mod world;

pub use context::Context;
pub use world::World;

use std::sync::Arc;

// pub trait Surface: Send + Sync + std::any::Any {
//     fn name(&self) -> &str;

//     fn on_start(&mut self, _ctx: &Context) {}
//     fn on_tick(&mut self, _ctx: &Context) {}
//     fn on_stop(&mut self, _ctx: &Context) {}
// }

pub trait Scene: Send + Sync + std::any::Any {
    fn name(&self) -> &'static str;

    /// Called when the scene is entered/started.
    fn on_enter(&mut self, _ctx: &Context) {}

    /// Called before rendering.
    fn on_tick(&mut self, _ctx: &Context) {}

    /// Called when the scene is exited/stopped.
    fn on_exit(&mut self, _ctx: &Context) {}

    /// Lookup a node in the scene.
    fn get(&self, id: &NodeId) -> Option<&Node>;

    /// Spawn/add a node to the scene.
    fn spawn(&mut self, node: Node) -> NodeId;

    /// Destroy/remove a node from the scene.
    fn destroy(&mut self, id: &NodeId) -> Option<Node>;
}

pub trait Entity: Send + Sync + std::any::Any {
    fn name(&self) -> &str;

    fn on_spawn(&mut self, _ctx: &Context) {}
    fn on_change(&mut self, _ctx: &Context) {}
    fn on_destroy(&mut self, _ctx: &Context) {}
}

pub trait Attribute: Send + Sync + std::any::Any {
    fn name(&self) -> &str;

    fn on_spawn(&mut self, _ctx: &Context, _node: &mut Node) {}
    fn on_change(&mut self, _ctx: &Context, _node: &mut Node) {}
    fn on_destroy(&mut self, _ctx: &Context, _node: &mut Node) {}
}

impl std::fmt::Debug for dyn Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.name())
    }
}

impl serde::Serialize for dyn Attribute {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name().serialize(s)
    }
}

pub trait Style {
    fn name(&self) -> &'static str;
    fn apply(&self, node: &mut Node);
}

impl std::fmt::Debug for dyn Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.name())
    }
}

impl serde::Serialize for dyn Style {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name().serialize(s)
    }
}

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
pub struct Node {
    pub id: NodeId,
    pub version: u32,
    pub name: String,
    pub styles: Vec<Arc<dyn Style>>,
    pub attributes: Vec<Arc<dyn Attribute>>,
    pub children: Vec<NodeId>,
}
