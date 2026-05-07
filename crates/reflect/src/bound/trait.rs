#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TraitBound {
    pub(crate) path: crate::Path,
    pub(crate) modifier: TraitBoundModifier,
}

impl TraitBound {
    pub fn new(path: &crate::Path, modifier: TraitBoundModifier) -> Self {
        return Self {
            path: path.clone(),
            modifier,
        };
    }

    pub fn to_bound(&self) -> crate::Bound {
        return crate::Bound::Trait(self.clone());
    }

    pub fn path(&self) -> &crate::Path {
        return &self.path;
    }

    pub fn modifier(&self) -> &TraitBoundModifier {
        return &self.modifier;
    }
}

impl std::fmt::Display for TraitBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}{}", &self.modifier, &self.path);
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TraitBoundModifier {
    None,

    /// `?`
    Maybe,
}

impl TraitBoundModifier {
    pub fn is_none(&self) -> bool {
        return match self {
            Self::None => true,
            _ => false,
        };
    }

    pub fn is_maybe(&self) -> bool {
        return match self {
            Self::Maybe => true,
            _ => false,
        };
    }
}

impl std::fmt::Display for TraitBoundModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::None => Ok(()),
            Self::Maybe => write!(f, "?"),
        };
    }
}
