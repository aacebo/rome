#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeParam {
    pub(crate) name: String,
    pub(crate) default: Option<crate::Type>,
    pub(crate) bounds: Vec<crate::Bound>,
}

impl TypeParam {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> crate::TypeParamBuilder {
        crate::TypeParamBuilder::new()
    }

    pub fn to_generic(&self) -> crate::Generic {
        crate::Generic::Type(self.clone())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bounds(&self) -> &[crate::Bound] {
        &self.bounds
    }

    pub fn default(&self) -> Option<&crate::Type> {
        match &self.default {
            None => None,
            Some(v) => Some(v),
        }
    }
}

impl std::fmt::Display for TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)?;

        if !self.bounds.is_empty() {
            write!(f, ": ")?;
        }

        for (i, bound) in self.bounds.iter().enumerate() {
            write!(f, "{}", bound)?;

            if i < self.bounds.len() - 1 {
                write!(f, " + ")?;
            }
        }

        if let Some(default) = &self.default {
            write!(f, " = {}", default)?;
        }

        Ok(())
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct TypeParamBuilder(crate::TypeParam);

impl Default for TypeParamBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeParamBuilder {
    pub fn new() -> Self {
        Self(crate::TypeParam {
            name: String::from(""),
            default: None,
            bounds: vec![],
        })
    }

    pub fn with_name(&self, name: &str) -> Self {
        let mut next = self.clone();
        next.0.name = name.to_string();
        next
    }

    pub fn with_default(&self, default: &crate::Type) -> Self {
        let mut next = self.clone();
        next.0.default = Some(default.clone());
        next
    }

    pub fn with_bounds(&self, bounds: &[crate::Bound]) -> Self {
        let mut next = self.clone();
        next.0.bounds.append(&mut bounds.to_vec());
        next
    }

    pub fn with_bound(&self, bound: &crate::Bound) -> Self {
        let mut next = self.clone();
        next.0.bounds.push(bound.clone());
        next
    }

    pub fn build(&self) -> crate::TypeParam {
        self.0.clone()
    }
}
