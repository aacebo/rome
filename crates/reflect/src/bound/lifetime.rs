#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LifetimeBound {
    pub(crate) name: String,
}

impl LifetimeBound {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
        };
    }

    pub fn to_bound(&self) -> crate::Bound {
        return crate::Bound::Lifetime(self.clone());
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }
}

impl std::fmt::Display for LifetimeBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "'{}", &self.name);
    }
}
