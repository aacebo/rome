#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    Type(crate::Type),
    Impl(crate::Impl),
}

impl Item {
    pub fn id(&self) -> crate::TypeId {
        return match self {
            Self::Type(v) => v.id(),
            Self::Impl(v) => v.id(),
        };
    }

    pub fn is_type(&self) -> bool {
        return match self {
            Self::Type(_) => true,
            _ => false,
        };
    }

    pub fn is_impl(&self) -> bool {
        return match self {
            Self::Impl(_) => true,
            _ => false,
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return match self {
            Self::Type(v) => v.clone(),
            _ => panic!("called 'to_type' on '{}'", self.id()),
        };
    }

    pub fn to_impl(&self) -> crate::Impl {
        return match self {
            Self::Impl(v) => v.clone(),
            _ => panic!("called 'to_impl' on '{}'", self.id()),
        };
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Type(v) => write!(f, "{}", v),
            Self::Impl(v) => write!(f, "{}", v),
        };
    }
}
