use crate::entity::Entity;

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
pub struct EventId(u64);

impl EventId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u64> for EventId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Event<'a> {
    pub id: EventId,
    pub action: Action,
    pub body: &'a Entity,
    pub time: chrono::DateTime<chrono::Utc>,
}
