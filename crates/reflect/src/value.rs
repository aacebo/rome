use crate::ToType;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Bool(bool),
    Number(crate::Number),
    Str(crate::Str<'a>),
    Slice(crate::Slice<'a>),
    Map(crate::Map<'a>),
    Mut(crate::Mut<'a>),
    Ref(crate::Ref<'a>),
    Dynamic(crate::Dynamic<'a>),
    Null,
}

impl<'a> Value<'a> {
    pub fn to_type(&self) -> crate::Type {
        match self {
            Self::Bool(v) => v.to_type(),
            Self::Number(v) => v.to_type(),
            Self::Str(v) => v.to_type(),
            Self::Slice(v) => v.to_type(),
            Self::Map(v) => v.to_type(),
            Self::Mut(v) => v.to_type(),
            Self::Ref(v) => v.to_type(),
            Self::Dynamic(v) => v.to_type(),
            Self::Null => panic!("called 'to_type' on '<null>'"),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Slice(v) => v.len(),
            Self::Map(v) => v.len(),
            Self::Str(v) => v.len(),
            Self::Mut(v) => v.len(),
            Self::Ref(v) => v.len(),
            v => panic!("called 'len' on '{}'", v.to_type()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Self> {
        match self {
            Self::Slice(v) => v.iter(),
            Self::Mut(v) => v.iter(),
            Self::Ref(v) => v.iter(),
            v => panic!("called 'iter' on '{}'", v.to_type()),
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }
    pub fn is_mut(&self) -> bool {
        matches!(self, Self::Mut(_))
    }
    pub fn is_ref(&self) -> bool {
        matches!(self, Self::Ref(_))
    }
    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }
    pub fn is_slice(&self) -> bool {
        matches!(self, Self::Slice(_))
    }
    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(_))
    }
    pub fn is_dynamic(&self) -> bool {
        matches!(self, Self::Dynamic(_))
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_bool(&self) -> &bool {
        match self {
            Self::Bool(v) => v,
            Self::Ref(v) => v.as_bool(),
            Self::Mut(v) => v.as_bool(),
            v => panic!("called 'as_bool' on '{}'", v.to_type()),
        }
    }

    pub fn as_number(&self) -> &crate::Number {
        match self {
            Self::Number(v) => v,
            Self::Ref(v) => v.as_number(),
            Self::Mut(v) => v.as_number(),
            v => panic!("called 'as_number' on '{}'", v.to_type()),
        }
    }

    pub fn as_str(&self) -> &crate::Str<'a> {
        match self {
            Self::Str(v) => v,
            v => panic!("called 'as_str' on '{}'", v.to_type()),
        }
    }

    pub fn as_slice(&self) -> &crate::Slice<'a> {
        match self {
            Self::Slice(v) => v,
            v => panic!("called 'as_slice' on '{}'", v.to_type()),
        }
    }

    pub fn as_dynamic(&self) -> &crate::Dynamic<'a> {
        match self {
            Self::Dynamic(v) => v,
            Self::Ref(v) => v.as_dynamic(),
            Self::Mut(v) => v.as_dynamic(),
            v => panic!("called 'as_dynamic' on '{}'", v.to_type()),
        }
    }

    pub fn as_map(&self) -> &crate::Map<'a> {
        match self {
            Self::Map(v) => v,
            v => panic!("called 'as_map' on '{}'", v.to_type()),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Self::Bool(v) => *v,
            Self::Ref(v) => v.to_bool(),
            Self::Mut(v) => v.to_bool(),
            v => panic!("called 'to_bool' on '{}'", v.to_type()),
        }
    }

    pub fn to_number(&self) -> crate::Number {
        match self {
            Self::Number(v) => *v,
            Self::Ref(v) => v.to_number(),
            Self::Mut(v) => v.to_number(),
            v => panic!("called 'to_number' on '{}'", v.to_type()),
        }
    }

    pub fn to_mut(&self) -> crate::Mut<'a> {
        match self {
            Self::Mut(v) => v.clone(),
            v => panic!("called 'to_mut' on '{}'", v.to_type()),
        }
    }

    pub fn to_ref(&self) -> crate::Ref<'a> {
        match self {
            Self::Ref(v) => v.clone(),
            v => panic!("called 'to_ref' on '{}'", v.to_type()),
        }
    }

    pub fn to_str(&self) -> crate::Str<'a> {
        match self {
            Self::Str(v) => v.clone(),
            v => panic!("called 'to_str' on '{}'", v.to_type()),
        }
    }

    pub fn to_slice(&self) -> crate::Slice<'a> {
        match self {
            Self::Slice(v) => v.clone(),
            v => panic!("called 'to_slice' on '{}'", v.to_type()),
        }
    }

    pub fn to_dynamic(&self) -> crate::Dynamic<'a> {
        match self {
            Self::Dynamic(v) => v.clone(),
            v => panic!("called 'to_dynamic' on '{}'", v.to_type()),
        }
    }

    pub fn to_map(&self) -> crate::Map<'a> {
        match self {
            Self::Map(v) => v.clone(),
            v => panic!("called 'to_map' on '{}'", v.to_type()),
        }
    }

    pub fn to_static(self) -> Value<'static> {
        match self {
            Self::Bool(v) => Value::Bool(v),
            Self::Number(v) => Value::Number(v),
            Self::Str(s) => {
                let leaked: &'static str = Box::leak(s.0.to_string().into_boxed_str());
                Value::Str(crate::Str(leaked))
            }
            Self::Slice(s) => Value::Slice(crate::Slice {
                ty: s.ty.clone(),
                value: s.value.into_iter().map(Value::to_static).collect(),
            }),
            Self::Map(m) => Value::Map(crate::Map {
                ty: m.ty.clone(),
                data: m
                    .data
                    .into_iter()
                    .map(|(k, v)| (k.to_static(), v.to_static()))
                    .collect(),
            }),
            Self::Mut(m) => m.value.clone().to_static(),
            Self::Ref(r) => r.value.clone().to_static(),
            Self::Dynamic(_) => Value::Null,
            Self::Null => Value::Null,
        }
    }
}

impl<'a> AsRef<Value<'a>> for Value<'a> {
    fn as_ref(&self) -> &Value<'a> {
        self
    }
}

impl<'a> crate::TypeOf for Value<'a> {
    fn type_of() -> crate::Type {
        crate::Type::Any
    }
}

impl<'a> crate::ToType for Value<'a> {
    fn to_type(&self) -> crate::Type {
        match self {
            Self::Bool(v) => v.to_type(),
            Self::Number(v) => v.to_type(),
            Self::Str(v) => v.to_type(),
            Self::Slice(v) => v.to_type(),
            Self::Map(v) => v.to_type(),
            Self::Mut(v) => v.to_type(),
            Self::Ref(v) => v.to_type(),
            Self::Dynamic(v) => v.to_type(),
            Self::Null => panic!("called 'ToType::to_type' on '<null>'"),
        }
    }
}

impl<'a> crate::ToValue for Value<'a> {
    fn to_value<'b>(&'b self) -> crate::Value<'b> {
        self.clone()
    }
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Bool(v) => other.is_bool() && other.as_bool() == v,
            Self::Number(v) => other.is_number() && other.as_number() == v,
            Self::Str(v) => other.is_str() && other.as_str() == v,
            Self::Slice(v) => other.is_slice() && other.as_slice() == v,
            Self::Map(v) => other.is_map() && other.as_map() == v,
            Self::Mut(v) => other.is_mut() && other.to_mut() == *v,
            Self::Ref(v) => other.is_ref() && other.to_ref() == *v,
            Self::Null => other.is_null(),
            _ => false,
        }
    }
}

impl<'a> std::ops::Index<usize> for Value<'a> {
    type Output = Self;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Slice(v) => v.index(index),
            Self::Ref(v) => v.index(index),
            Self::Mut(v) => v.index(index),
            _ => panic!("called 'Index<usize>::index' on '{}'", self.to_type()),
        }
    }
}

impl<'a> std::ops::Index<&'a str> for Value<'a> {
    type Output = Self;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self {
            Self::Map(v) => v.index(&crate::Value::Str(crate::Str(index))),
            _ => panic!("called 'Index<&str>::index' on '{}'", self.to_type()),
        }
    }
}

impl<'a> std::ops::Index<&Self> for Value<'a> {
    type Output = Self;

    fn index(&self, index: &Self) -> &Self::Output {
        match self {
            Self::Map(v) => v.index(index),
            Self::Slice(v) => v.index(index.to_i32() as usize),
            _ => panic!("called 'Index<&Value>::index' on '{}'", self.to_type()),
        }
    }
}

impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::Str(v) => write!(f, "{}", v),
            Self::Slice(v) => write!(f, "{}", v),
            Self::Map(v) => write!(f, "{}", v),
            Self::Mut(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Dynamic(v) => write!(f, "{}", v),
            Self::Null => write!(f, "<null>"),
        }
    }
}

impl<'a> Eq for Value<'a> {}

impl<'a> Ord for Value<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = match self {
            Self::Bool(v) => v.partial_cmp(other.as_bool()),
            Self::Number(v) => v.partial_cmp(other.as_number()),
            Self::Str(v) => v.0.partial_cmp(other.as_str().0),
            Self::Mut(v) => v.as_ref().partial_cmp(other.to_mut().as_ref()),
            Self::Ref(v) => v.as_ref().partial_cmp(other.to_ref().as_ref()),
            _ => None,
        };

        match ord {
            None => panic!("called 'cmp' on '{}'", self.to_type()),
            Some(v) => v,
        }
    }
}

impl<'a> PartialOrd for Value<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for Value<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bool(v) => v.serialize(s),
            Self::Number(v) => v.serialize(s),
            Self::Str(v) => v.0.serialize(s),
            Self::Slice(v) => v.value.serialize(s),
            Self::Map(v) => v.serialize(s),
            Self::Mut(v) => v.value.serialize(s),
            Self::Ref(v) => v.value.serialize(s),
            Self::Dynamic(v) => v.serialize(s),
            Self::Null => s.serialize_none(),
        }
    }
}
