use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct TypeId(String);

impl TypeId {
    pub(crate) fn from_str(value: &str) -> Self {
        Self(value.to_string())
    }

    pub(crate) fn from_string(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Eq for TypeId {}

impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<&str> for TypeId {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<String> for TypeId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl AsRef<TypeId> for TypeId {
    fn as_ref(&self) -> &TypeId {
        self
    }
}

impl AsMut<TypeId> for TypeId {
    fn as_mut(&mut self) -> &mut TypeId {
        self
    }
}

impl Deref for TypeId {
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl DerefMut for TypeId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}
