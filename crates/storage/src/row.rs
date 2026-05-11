use crate::Version;

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
