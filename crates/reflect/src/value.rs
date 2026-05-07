use std::ops::Index;

use crate::{ToType, ToValue};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Bool(bool),
    Number(crate::Number),
    Str(crate::Str),
    Slice(crate::Slice),
    Map(crate::Map),
    Mut(crate::Mut),
    Ref(crate::Ref),
    Dynamic(crate::Dynamic),
    Null,
}

impl Value {
    pub fn to_type(&self) -> crate::Type {
        return match self {
            Self::Bool(v) => v.to_type(),
            Self::Number(v) => v.to_type(),
            Self::Str(v) => v.to_type(),
            Self::Slice(v) => v.to_type(),
            Self::Map(v) => v.to_type(),
            Self::Mut(v) => v.to_type(),
            Self::Ref(v) => v.to_type(),
            Self::Dynamic(v) => v.to_type(),
            Self::Null => panic!("called 'to_type' on '<null>'"),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::Slice(v) => v.len(),
            Self::Map(v) => v.len(),
            Self::Str(v) => v.len(),
            Self::Mut(v) => v.len(),
            Self::Ref(v) => v.len(),
            v => panic!("called 'len' on '{}'", v.to_type()),
        };
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Self> {
        return match self {
            Self::Slice(v) => v.iter(),
            Self::Mut(v) => v.iter(),
            Self::Ref(v) => v.iter(),
            v => panic!("called 'len' on '{}'", v.to_type()),
        };
    }

    pub fn is_bool(&self) -> bool {
        return match self {
            Self::Bool(_) => true,
            _ => false,
        };
    }

    pub fn is_number(&self) -> bool {
        return match self {
            Self::Number(_) => true,
            _ => false,
        };
    }

    pub fn is_mut(&self) -> bool {
        return match self {
            Self::Mut(_) => true,
            _ => false,
        };
    }

    pub fn is_ref(&self) -> bool {
        return match self {
            Self::Ref(_) => true,
            _ => false,
        };
    }

    pub fn is_str(&self) -> bool {
        return match self {
            Self::Str(_) => true,
            _ => false,
        };
    }

    pub fn is_slice(&self) -> bool {
        return match self {
            Self::Slice(_) => true,
            _ => false,
        };
    }

    pub fn is_map(&self) -> bool {
        return match self {
            Self::Map(_) => true,
            _ => false,
        };
    }

    pub fn is_dynamic(&self) -> bool {
        return match self {
            Self::Dynamic(_) => true,
            _ => false,
        };
    }

    pub fn is_null(&self) -> bool {
        return match self {
            Self::Null => true,
            _ => false,
        };
    }

    pub fn as_bool(&self) -> &bool {
        return match self {
            Self::Bool(v) => v,
            Self::Ref(v) => v.as_bool(),
            Self::Mut(v) => v.as_bool(),
            v => panic!("called 'as_bool' on '{}'", v.to_type()),
        };
    }

    pub fn as_number(&self) -> &crate::Number {
        return match self {
            Self::Number(v) => v,
            Self::Ref(v) => v.as_number(),
            Self::Mut(v) => v.as_number(),
            v => panic!("called 'as_number' on '{}'", v.to_type()),
        };
    }

    pub fn as_str(&self) -> &crate::Str {
        return match self {
            Self::Str(v) => v,
            Self::Ref(v) => v.as_str(),
            Self::Mut(v) => v.as_str(),
            v => panic!("called 'as_str' on '{}'", v.to_type()),
        };
    }

    pub fn as_slice(&self) -> &crate::Slice {
        return match self {
            Self::Slice(v) => v,
            Self::Ref(v) => v.as_slice(),
            Self::Mut(v) => v.as_slice(),
            v => panic!("called 'as_slice' on '{}'", v.to_type()),
        };
    }

    pub fn as_dynamic(&self) -> &crate::Dynamic {
        return match self {
            Self::Dynamic(v) => v,
            Self::Ref(v) => v.as_dynamic(),
            Self::Mut(v) => v.as_dynamic(),
            v => panic!("called 'as_dynamic' on '{}'", v.to_type()),
        };
    }

    pub fn as_map(&self) -> &crate::Map {
        return match self {
            Self::Map(v) => v,
            Self::Ref(v) => v.as_map(),
            Self::Mut(v) => v.as_map(),
            v => panic!("called 'as_map' on '{}'", v.to_type()),
        };
    }

    pub fn to_bool(&self) -> bool {
        return match self {
            Self::Bool(v) => *v,
            Self::Ref(v) => v.to_bool(),
            Self::Mut(v) => v.to_bool(),
            v => panic!("called 'to_bool' on '{}'", v.to_type()),
        };
    }

    pub fn to_number(&self) -> crate::Number {
        return match self {
            Self::Number(v) => v.clone(),
            Self::Ref(v) => v.to_number(),
            Self::Mut(v) => v.to_number(),
            v => panic!("called 'to_number' on '{}'", v.to_type()),
        };
    }

    pub fn to_mut(&self) -> crate::Mut {
        return match self {
            Self::Mut(v) => v.clone(),
            Self::Ref(v) => v.to_mut(),
            v => panic!("called 'to_mut' on '{}'", v.to_type()),
        };
    }

    pub fn to_ref(&self) -> crate::Ref {
        return match self {
            Self::Ref(v) => v.clone(),
            Self::Mut(v) => v.to_ref(),
            v => panic!("called 'to_ref' on '{}'", v.to_type()),
        };
    }

    pub fn to_str(&self) -> crate::Str {
        return match self {
            Self::Str(v) => v.clone(),
            Self::Ref(v) => v.to_str(),
            Self::Mut(v) => v.to_str(),
            v => panic!("called 'to_str' on '{}'", v.to_type()),
        };
    }

    pub fn to_slice(&self) -> crate::Slice {
        return match self {
            Self::Slice(v) => v.clone(),
            Self::Ref(v) => v.to_slice(),
            Self::Mut(v) => v.to_slice(),
            v => panic!("called 'to_slice' on '{}'", v.to_type()),
        };
    }

    pub fn to_dynamic(&self) -> crate::Dynamic {
        return match self {
            Self::Dynamic(v) => v.clone(),
            Self::Ref(v) => v.to_dynamic(),
            Self::Mut(v) => v.to_dynamic(),
            v => panic!("called 'to_dynamic' on '{}'", v.to_type()),
        };
    }

    pub fn to_map(&self) -> crate::Map {
        return match self {
            Self::Map(v) => v.clone(),
            Self::Ref(v) => v.to_map(),
            Self::Mut(v) => v.to_map(),
            v => panic!("called 'to_map' on '{}'", v.to_type()),
        };
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Value {
        return self;
    }
}

impl crate::TypeOf for Value {
    fn type_of() -> crate::Type {
        return crate::Type::Any;
    }
}

impl crate::ToType for Value {
    fn to_type(&self) -> crate::Type {
        return match self {
            Self::Bool(v) => v.to_type(),
            Self::Number(v) => v.to_type(),
            Self::Str(v) => v.to_type(),
            Self::Slice(v) => v.to_type(),
            Self::Map(v) => v.to_type(),
            Self::Mut(v) => v.to_type(),
            Self::Ref(v) => v.to_type(),
            Self::Dynamic(v) => v.to_type(),
            Self::Null => panic!("called 'ToType::to_type' on '<null>'"),
        };
    }
}

impl crate::ToValue for Value {
    fn to_value(self) -> crate::Value {
        return self;
    }
}

impl crate::AsValue for Value {
    fn as_value(&self) -> crate::Value {
        return self.clone();
    }
}

impl crate::Dyn for Value {}

impl crate::Object for Value {
    fn field(&self, name: &crate::FieldName) -> crate::Value {
        return match self {
            Self::Dynamic(v) => v.field(name),
            v => panic!("called 'zinq::reflect::Object::field' on '{}'", v.to_type()),
        };
    }
}

impl crate::Sequence for Value {
    fn len(&self) -> usize {
        return match self {
            Self::Dynamic(v) => v.len(),
            v => panic!("called 'zinq::reflect::Sequence::len' on '{}'", v.to_type()),
        };
    }

    fn index(&self, i: usize) -> crate::Value {
        return match self {
            Self::Dynamic(v) => v.index(i).clone(),
            v => panic!(
                "called 'zinq::reflect::Sequence::index' on '{}'",
                v.to_type()
            ),
        };
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Self::Bool(v) => other.is_bool() && other.as_bool() == v,
            Self::Number(v) => other.is_number() && other.as_number() == v,
            Self::Str(v) => other.is_str() && other.as_str() == v,
            Self::Slice(v) => other.is_slice() && other.as_slice() == v,
            Self::Map(v) => other.is_map() && other.as_map() == v,
            Self::Mut(v) => other.is_mut() && other.to_mut() == *v,
            Self::Ref(v) => other.is_ref() && other.to_ref() == *v,
            Self::Null => other.is_null(),
            _ => false,
        };
    }
}

impl std::ops::Index<usize> for Value {
    type Output = Self;

    fn index(&self, index: usize) -> &Self::Output {
        return match self {
            Self::Slice(v) => v.index(index),
            Self::Ref(v) => v.index(index),
            Self::Mut(v) => v.index(index),
            _ => panic!("called 'Index<usize>::index' on '{}'", self.to_type()),
        };
    }
}

impl std::ops::IndexMut<usize> for Value {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match self {
            Self::Slice(v) => v.index_mut(index),
            Self::Ref(v) => v.index_mut(index),
            Self::Mut(v) => v.index_mut(index),
            _ => panic!(
                "called 'IndexMut<usize>::index_mut' on '{}'",
                self.to_type()
            ),
        };
    }
}

impl std::ops::Index<&str> for Value {
    type Output = Self;

    fn index(&self, index: &str) -> &Self::Output {
        return match self {
            Self::Map(v) => v.index(&index.to_value()),
            Self::Ref(v) => v.index(&index.to_value()),
            Self::Mut(v) => v.index(&index.to_value()),
            _ => panic!("called 'Index<&str>::index' on '{}'", self.to_type()),
        };
    }
}

impl std::ops::Index<&Self> for Value {
    type Output = Self;

    fn index(&self, index: &Self) -> &Self::Output {
        return match self {
            Self::Map(v) => v.index(index),
            Self::Slice(v) => v.index(index.to_i32() as usize),
            Self::Ref(v) => v.index(index),
            Self::Mut(v) => v.index(index),
            _ => panic!("called 'Index<&Value>::index' on '{}'", self.to_type()),
        };
    }
}

impl std::ops::IndexMut<&Self> for Value {
    fn index_mut(&mut self, index: &Self) -> &mut Self::Output {
        return match self {
            Self::Map(v) => v.index_mut(index),
            Self::Slice(v) => v.index_mut(index.to_i32() as usize),
            Self::Ref(v) => v.index_mut(index),
            Self::Mut(v) => v.index_mut(index),
            _ => panic!(
                "called 'IndexMut<&Value>::index_mut' on '{}'",
                self.to_type()
            ),
        };
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::Str(v) => write!(f, "{}", v),
            Self::Slice(v) => write!(f, "{}", v),
            Self::Map(v) => write!(f, "{}", v),
            Self::Mut(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Dynamic(v) => write!(f, "{}", v),
            Self::Null => write!(f, "<null>"),
        };
    }
}

impl Eq for Value {}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return match self.partial_cmp(other) {
            None => panic!("called 'PartialOrd::partial_cmp' on '{}'", self.to_type()),
            Some(v) => v,
        };
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return match self {
            Self::Bool(v) => Some(v.partial_cmp(other.as_bool())?),
            Self::Number(v) => Some(v.partial_cmp(other.as_number())?),
            Self::Str(v) => Some(v.partial_cmp(other.as_str())?),
            Self::Mut(v) => Some(v.partial_cmp(other.to_mut().as_ref())?),
            Self::Ref(v) => Some(v.partial_cmp(other.to_ref().as_ref())?),
            _ => None,
        };
    }
}
