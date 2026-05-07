mod r#const;
mod lifetime;
mod r#type;

pub use r#const::*;
pub use lifetime::*;
pub use r#type::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Generics(pub(crate) Vec<Generic>);

impl Generics {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Generic> {
        return self.0.iter();
    }

    pub fn add(&mut self, param: &Generic) {
        self.0.push(param.clone());
    }
}

impl<const N: usize> From<[Generic; N]> for Generics {
    fn from(value: [Generic; N]) -> Self {
        return Self(value.to_vec());
    }
}

impl std::ops::Index<usize> for Generics {
    type Output = Generic;

    fn index(&self, index: usize) -> &Self::Output {
        return self.0.index(index);
    }
}

impl std::ops::IndexMut<usize> for Generics {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.0.index_mut(index);
    }
}

impl std::fmt::Display for Generics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.len() > 0 {
            write!(f, "<")?;

            for (i, param) in self.0.iter().enumerate() {
                write!(f, "{}", param)?;

                if i < self.0.len() - 1 {
                    write!(f, ", ")?;
                }
            }

            write!(f, ">")?;
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Generic {
    Const(ConstParam),
    Lifetime(LifetimeParam),
    Type(TypeParam),
}

impl Generic {
    pub fn name(&self) -> &str {
        return match self {
            Self::Const(_) => "const",
            Self::Lifetime(_) => "lifetime",
            Self::Type(_) => "type",
        };
    }

    pub fn is_const(&self) -> bool {
        return match self {
            Self::Const(_) => true,
            _ => false,
        };
    }

    pub fn is_lifetime(&self) -> bool {
        return match self {
            Self::Lifetime(_) => true,
            _ => false,
        };
    }

    pub fn is_type(&self) -> bool {
        return match self {
            Self::Type(_) => true,
            _ => false,
        };
    }

    pub fn to_const(&self) -> ConstParam {
        return match self {
            Self::Const(v) => v.clone(),
            _ => panic!("called 'to_const' on '{}'", self.name()),
        };
    }

    pub fn to_lifetime(&self) -> LifetimeParam {
        return match self {
            Self::Lifetime(v) => v.clone(),
            _ => panic!("called 'to_lifetime' on '{}'", self.name()),
        };
    }

    pub fn to_type(&self) -> TypeParam {
        return match self {
            Self::Type(v) => v.clone(),
            _ => panic!("called 'to_type' on '{}'", self.name()),
        };
    }
}

impl std::fmt::Display for Generic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Const(v) => write!(f, "{}", v),
            Self::Lifetime(v) => write!(f, "{}", v),
            Self::Type(v) => write!(f, "{}", v),
        };
    }
}
