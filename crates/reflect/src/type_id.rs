use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct TypeId(Rc<str>);

thread_local! {
    static INTERNED: std::cell::RefCell<std::collections::HashMap<&'static str, Rc<str>>>
        = std::cell::RefCell::new(std::collections::HashMap::new());
}

impl TypeId {
    pub(crate) fn from_str(value: &'static str) -> Self {
        let rc = INTERNED.with(|m| {
            let mut map = m.borrow_mut();
            if let Some(existing) = map.get(value) {
                existing.clone()
            } else {
                let rc: Rc<str> = Rc::from(value);
                map.insert(value, rc.clone());
                rc
            }
        });
        Self(rc)
    }

    pub(crate) fn from_string(value: String) -> Self {
        Self(Rc::from(value.as_str()))
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
        Rc::ptr_eq(&self.0, &other.0) || *self.0 == *other.0
    }
}

impl PartialEq<&str> for TypeId {
    fn eq(&self, other: &&str) -> bool {
        &*self.0 == *other
    }
}

impl PartialEq<String> for TypeId {
    fn eq(&self, other: &String) -> bool {
        &*self.0 == other.as_str()
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
