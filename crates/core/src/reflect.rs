use std::{any::Any, collections::BTreeMap};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Self>),
    Map(BTreeMap<String, Self>),
}

impl Value {
    pub fn type_id(&self) -> std::any::TypeId {
        match self {
            Self::Bool(v) => v.type_id(),
            Self::Int(v) => v.type_id(),
            Self::Float(v) => v.type_id(),
            Self::String(v) => v.type_id(),
            Self::Array(v) => v.type_id(),
            Self::Map(v) => v.type_id(),
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, Self>> {
        match self {
            Self::Map(v) => Some(v),
            _ => None,
        }
    }
}
