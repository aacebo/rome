mod float;
mod int;

pub use float::*;
pub use int::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Number {
    Int(Int),
    Float(Float),
}

impl Number {
    pub fn to_type(&self) -> crate::Type {
        return match self {
            Self::Int(v) => v.to_type(),
            Self::Float(v) => v.to_type(),
        };
    }

    pub fn is_int(&self) -> bool {
        return match self {
            Self::Int(_) => true,
            _ => false,
        };
    }

    pub fn is_float(&self) -> bool {
        return match self {
            Self::Float(_) => true,
            _ => false,
        };
    }

    pub fn to_int(&self) -> Int {
        return match self {
            Self::Int(v) => *v,
            v => panic!("called 'to_int' on '{}'", v.to_type()),
        };
    }

    pub fn as_int(&self) -> &Int {
        return match self {
            Self::Int(v) => v,
            v => panic!("called 'as_int' on '{}'", v.to_type()),
        };
    }

    pub fn to_float(&self) -> Float {
        return match self {
            Self::Float(v) => *v,
            v => panic!("called 'to_float' on '{}'", v.to_type()),
        };
    }

    pub fn as_float(&self) -> &Float {
        return match self {
            Self::Float(v) => v,
            v => panic!("called 'as_float' on '{}'", v.to_type()),
        };
    }

    pub fn set_int(&mut self, value: Int) {
        *self = Self::Int(value);
    }

    pub fn set_float(&mut self, value: Float) {
        *self = Self::Float(value);
    }
}

impl crate::ToValue for Number {
    fn to_value(self) -> crate::Value {
        return crate::Value::Number(self.clone());
    }
}

impl crate::AsValue for Number {
    fn as_value(&self) -> crate::Value {
        return crate::Value::Number(self.clone());
    }
}

impl PartialEq<crate::Value> for Number {
    fn eq(&self, other: &crate::Value) -> bool {
        return other.is_number() && other.as_number() == self;
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
        };
    }
}
