#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeParam {
    pub(crate) name: String,
    pub(crate) default: Option<crate::Type>,
    pub(crate) bounds: Vec<crate::Bound>,
}

impl TypeParam {
    pub fn new() -> crate::TypeParamBuilder {
        return crate::TypeParamBuilder::new();
    }

    pub fn to_generic(&self) -> crate::Generic {
        return crate::Generic::Type(self.clone());
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn bounds(&self) -> &[crate::Bound] {
        return &self.bounds;
    }

    pub fn default(&self) -> Option<&crate::Type> {
        return match &self.default {
            None => None,
            Some(v) => Some(v),
        };
    }
}

impl std::fmt::Display for TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)?;

        if self.bounds.len() > 0 {
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

        return Ok(());
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct TypeParamBuilder(crate::TypeParam);

impl TypeParamBuilder {
    pub fn new() -> Self {
        return Self(crate::TypeParam {
            name: String::from(""),
            default: None,
            bounds: vec![],
        });
    }

    pub fn with_name(&self, name: &str) -> Self {
        let mut next = self.clone();
        next.0.name = name.to_string();
        return next;
    }

    pub fn with_default(&self, default: &crate::Type) -> Self {
        let mut next = self.clone();
        next.0.default = Some(default.clone());
        return next;
    }

    pub fn with_bounds(&self, bounds: &[crate::Bound]) -> Self {
        let mut next = self.clone();
        next.0.bounds.append(&mut bounds.to_vec());
        return next;
    }

    pub fn with_bound(&self, bound: &crate::Bound) -> Self {
        let mut next = self.clone();
        next.0.bounds.push(bound.clone());
        return next;
    }

    pub fn build(&self) -> crate::TypeParam {
        return self.0.clone();
    }
}
