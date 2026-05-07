use std::sync::Arc;

mod r#dyn;
mod object;
mod sequence;

pub use r#dyn::*;
pub use object::*;
pub use sequence::*;

#[derive(Debug, Clone)]
pub enum Dynamic {
    Dyn(Arc<dyn crate::Dyn>),
    Object(Arc<dyn crate::Object>),
    Sequence(Arc<dyn crate::Sequence>),
}

impl Dynamic {
    pub fn from_dyn<T: crate::Dyn>(value: T) -> Self {
        Self::Dyn(Arc::new(value))
    }

    pub fn from_object<T: crate::Object>(value: T) -> Self {
        Self::Object(Arc::new(value))
    }

    pub fn from_sequence<T: crate::Sequence>(value: T) -> Self {
        Self::Sequence(Arc::new(value))
    }

    pub fn to_type(&self) -> crate::Type {
        match self {
            Self::Dyn(v) => v.to_type(),
            Self::Object(v) => v.to_type(),
            Self::Sequence(v) => v.to_type(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Sequence(v) => v.len(),
            v => panic!("called 'len' on '{}'", v.to_type()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    pub fn is_sequence(&self) -> bool {
        matches!(self, Self::Sequence(_))
    }

    pub fn as_dyn(&self) -> &dyn crate::Dyn {
        match self {
            Self::Dyn(v) => v.as_ref(),
            Self::Object(v) => v.as_ref(),
            Self::Sequence(v) => v.as_ref(),
        }
    }

    pub fn as_object(&self) -> &dyn crate::Object {
        match self {
            Self::Object(v) => v.as_ref(),
            v => panic!("called 'as_object' on '{}'", v.to_type()),
        }
    }

    pub fn as_object_mut(&mut self) -> &mut dyn crate::Object {
        match self {
            Self::Object(v) => Arc::get_mut(v).unwrap(),
            v => panic!("called 'as_object_mut' on '{}'", v.to_type()),
        }
    }

    pub fn as_sequence(&self) -> &dyn crate::Sequence {
        match self {
            Self::Sequence(v) => v.as_ref(),
            v => panic!("called 'as_sequence' on '{}'", v.to_type()),
        }
    }

    pub fn as_sequence_mut(&mut self) -> &mut dyn crate::Sequence {
        match self {
            Self::Sequence(v) => Arc::get_mut(v).unwrap(),
            v => panic!("called 'as_sequence_mut' on '{}'", v.to_type()),
        }
    }

    pub fn is<T: crate::Object>(&self) -> bool {
        match self {
            Self::Dyn(v) => v.is::<T>(),
            Self::Object(v) => v.is::<T>(),
            Self::Sequence(v) => v.is::<T>(),
        }
    }

    pub fn to<T: crate::Object>(&self) -> &T {
        match self {
            Self::Dyn(v) => v.downcast_ref::<T>().unwrap(),
            Self::Object(v) => v.downcast_ref::<T>().unwrap(),
            Self::Sequence(v) => v.downcast_ref::<T>().unwrap(),
        }
    }
}

impl crate::TypeOf for Dynamic {
    fn type_of() -> crate::Type {
        crate::Type::Any
    }
}

impl crate::ToType for Dynamic {
    fn to_type(&self) -> crate::Type {
        match self {
            Self::Dyn(v) => v.to_type(),
            Self::Object(v) => v.to_type(),
            Self::Sequence(v) => v.to_type(),
        }
    }
}

impl crate::ToValue for Dynamic {
    fn to_value(self) -> crate::Value {
        crate::Value::Dynamic(self.clone())
    }
}

impl crate::AsValue for Dynamic {
    fn as_value(&self) -> crate::Value {
        crate::Value::Dynamic(self.clone())
    }
}

impl AsRef<dyn crate::Object> for Dynamic {
    fn as_ref(&self) -> &dyn crate::Object {
        match self {
            Self::Object(v) => v.as_ref(),
            v => panic!("called 'AsRef<dyn Object>::as_ref' on '{}'", v.to_type()),
        }
    }
}

impl AsMut<dyn crate::Object> for Dynamic {
    fn as_mut(&mut self) -> &mut dyn crate::Object {
        match self {
            Self::Object(v) => Arc::get_mut(v).unwrap(),
            v => panic!("called 'AsMut<dyn Object>::as_mut' on '{}'", v.to_type()),
        }
    }
}

impl AsRef<dyn crate::Sequence> for Dynamic {
    fn as_ref(&self) -> &dyn crate::Sequence {
        match self {
            Self::Sequence(v) => v.as_ref(),
            v => panic!("called 'AsRef<dyn Sequence>::as_ref' on '{}'", v.to_type()),
        }
    }
}

impl AsMut<dyn crate::Sequence> for Dynamic {
    fn as_mut(&mut self) -> &mut dyn crate::Sequence {
        match self {
            Self::Sequence(v) => Arc::get_mut(v).unwrap(),
            v => panic!("called 'AsMut<dyn Sequence>::as_mut' on '{}'", v.to_type()),
        }
    }
}

impl std::ops::Deref for Dynamic {
    type Target = dyn crate::Object;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl std::ops::DerefMut for Dynamic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl std::ops::Index<usize> for Dynamic {
    type Output = crate::Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Sequence(v) => v.index_ref(index),
            v => panic!("called 'std::ops::Index<usize>' on {}", v.to_type()),
        }
    }
}

impl std::fmt::Display for Dynamic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dyn(v) => write!(f, "{}", v.as_ref()),
            Self::Object(v) => write!(f, "{}", v.as_ref()),
            Self::Sequence(v) => write!(f, "{}", v.as_ref()),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Dynamic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Dyn(v) => v.serialize(serializer),
            Self::Object(v) => v.serialize(serializer),
            Self::Sequence(v) => v.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Dynamic {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}
