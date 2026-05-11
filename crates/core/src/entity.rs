use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use ayr_storage::{Row, Table};

use super::Context;

pub trait Entity: Send + Sync + std::fmt::Debug + std::any::Any + ayr_reflect::ToValue {
    fn name(&self) -> &'static str;

    fn on_spawn(&mut self, _ctx: &Context) {}
    fn on_change(&mut self, _ctx: &Context) {}
    fn on_destroy(&mut self, _ctx: &Context) {}
}

impl serde::Serialize for dyn Entity {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_value().serialize(s)
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
pub struct EntityId(u64);

impl EntityId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u64> for EntityId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "entity_{}", &self.0)
    }
}

#[derive(Default)]
pub struct EntityTable {
    next_id: AtomicU64,
    rows: HashMap<EntityId, Row<EntityId, Arc<dyn Entity>>>,
}

impl EntityTable {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::default(),
            rows: HashMap::new(),
        }
    }
}

impl Table for EntityTable {
    type Id = EntityId;
    type Data = Arc<dyn Entity>;

    fn exists(&self, id: &Self::Id) -> bool {
        self.rows.contains_key(id)
    }

    fn get(&self, id: &Self::Id) -> Option<&Row<Self::Id, Self::Data>> {
        self.rows.get(id)
    }

    fn insert(&mut self, data: Self::Data) -> &Row<Self::Id, Self::Data> {
        let id = EntityId::from(self.next_id.fetch_add(1, Ordering::Relaxed));
        self.rows.insert(id, Row::new(id, data));
        self.rows.get(&id).unwrap()
    }

    fn update<P>(&mut self, id: &Self::Id, project: P)
    where
        P: FnOnce(&mut Self::Data),
    {
        let row = match self.rows.get_mut(id) {
            None => return,
            Some(v) => v,
        };

        project(&mut row.data);
        row.version.increment();
        row.updated_at = std::time::SystemTime::now();
    }

    fn delete(&mut self, id: &Self::Id) -> Option<Row<Self::Id, Self::Data>> {
        self.rows.remove(id)
    }
}
