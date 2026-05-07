mod lifetime;
mod r#trait;

pub use lifetime::*;
pub use r#trait::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Bound {
    Trait(TraitBound),
    Lifetime(LifetimeBound),
}

impl Bound {
    pub fn name(&self) -> String {
        return match self {
            Self::Trait(v) => v.path.to_string(),
            Self::Lifetime(v) => v.name.clone(),
        };
    }

    pub fn is_trait(&self) -> bool {
        return match self {
            Self::Trait(_) => true,
            _ => false,
        };
    }

    pub fn is_lifetime(&self) -> bool {
        return match self {
            Self::Lifetime(_) => true,
            _ => false,
        };
    }

    pub fn to_trait(&self) -> TraitBound {
        return match self {
            Self::Trait(v) => v.clone(),
            _ => panic!("called 'to_trait' on '{}'", self.name()),
        };
    }

    pub fn to_lifetime(&self) -> LifetimeBound {
        return match self {
            Self::Lifetime(v) => v.clone(),
            _ => panic!("called 'to_lifetime' on '{}'", self.name()),
        };
    }
}

impl std::fmt::Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Trait(v) => write!(f, "{}", v),
            Self::Lifetime(v) => write!(f, "{}", v),
        };
    }
}
