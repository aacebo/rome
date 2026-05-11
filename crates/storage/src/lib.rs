use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, serde::Serialize)]
pub struct Row<Id, Data> {
    pub id: Id,
    pub version: Version,
    pub data: Data,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
}

impl<Id, Data> Row<Id, Data> {
    pub fn new(id: Id, data: Data) -> Self {
        Self {
            id,
            version: Version::default(),
            data,
            created_at: std::time::SystemTime::now(),
            updated_at: std::time::SystemTime::now(),
        }
    }
}

pub trait Table {
    type Id;
    type Data;

    fn exists(&self, id: &Self::Id) -> bool;
    fn get(&self, id: &Self::Id) -> Option<&Row<Self::Id, Self::Data>>;
    fn insert(&mut self, data: Self::Data) -> &Row<Self::Id, Self::Data>;
    fn update<P>(&mut self, id: &Self::Id, project: P)
    where
        P: FnOnce(&mut Self::Data);
    fn delete(&mut self, id: &Self::Id) -> Option<Row<Self::Id, Self::Data>>;
}

#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct Version(AtomicU64);

impl Version {
    pub fn to_u64(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    pub fn increment(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self(AtomicU64::new(value))
    }
}

impl Default for Version {
    fn default() -> Self {
        Self(AtomicU64::new(0))
    }
}

impl Eq for Version {}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.to_u64() == other.to_u64()
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_u64().cmp(&other.to_u64())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u64())
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u64())
    }
}
