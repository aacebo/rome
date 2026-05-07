#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Visibility {
    Public(Public),
    Private,
}

impl Visibility {
    pub fn is_public(&self) -> bool {
        return match self {
            Self::Public(_) => true,
            _ => false,
        };
    }

    pub fn is_private(&self) -> bool {
        return match self {
            Self::Private => true,
            _ => false,
        };
    }
}

impl Default for Visibility {
    fn default() -> Self {
        return Visibility::Private;
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Public(v) => write!(f, "{}", v),
            Self::Private => Ok(()),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Public {
    Full,
    Type,
    Super,
    Crate,
    Mod(String),
}

impl Public {
    pub fn is_full(&self) -> bool {
        return match self {
            Self::Full => true,
            _ => false,
        };
    }

    pub fn is_type(&self) -> bool {
        return match self {
            Self::Type => true,
            _ => false,
        };
    }

    pub fn is_super(&self) -> bool {
        return match self {
            Self::Super => true,
            _ => false,
        };
    }

    pub fn is_crate(&self) -> bool {
        return match self {
            Self::Crate => true,
            _ => false,
        };
    }

    pub fn is_mod(&self) -> bool {
        return match self {
            Self::Mod(_) => true,
            _ => false,
        };
    }
}

impl std::fmt::Display for Public {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Full => write!(f, "pub"),
            Self::Type => write!(f, "pub(self)"),
            Self::Super => write!(f, "pub(super)"),
            Self::Crate => write!(f, "pub(crate)"),
            Self::Mod(path) => write!(f, "pub(in {})", path),
        };
    }
}
