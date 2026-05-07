#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Layout {
    Unit,
    Key,
    Index,
}

impl Layout {
    pub fn is_unit(&self) -> bool {
        return match self {
            Self::Unit => true,
            _ => false,
        };
    }

    pub fn is_key(&self) -> bool {
        return match self {
            Self::Key => true,
            _ => false,
        };
    }

    pub fn is_index(&self) -> bool {
        return match self {
            Self::Index => true,
            _ => false,
        };
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Unit => write!(f, "unit"),
            Self::Key => write!(f, "key"),
            Self::Index => write!(f, "index"),
        };
    }
}
