pub mod styles;
pub mod vec;

use crate::NodeId;

pub trait Style {
    fn name(&self) -> &'static str;
    fn apply(&self, node: &mut NodeId);
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
