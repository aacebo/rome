use crate::{Param, Visibility};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Method {
    pub(crate) meta: crate::MetaData,
    pub(crate) is_async: bool,
    pub(crate) vis: Visibility,
    pub(crate) name: String,
    pub(crate) generics: crate::Generics,
    pub(crate) params: Vec<Param>,
    pub(crate) return_type: Box<crate::Type>,
}

impl Method {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> crate::MethodBuilder {
        crate::MethodBuilder::new()
    }

    pub fn meta(&self) -> &crate::MetaData {
        &self.meta
    }

    pub fn is_async(&self) -> bool {
        self.is_async
    }

    pub fn vis(&self) -> Visibility {
        self.vis.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn generics(&self) -> &crate::Generics {
        &self.generics
    }

    pub fn params(&self) -> &[Param] {
        &self.params
    }

    pub fn has_param(&self, name: &str) -> bool {
        self.params.iter().any(|v| v.name() == name)
    }

    pub fn param(&self, name: &str) -> &Param {
        self.params.iter().find(|v| v.name() == name).unwrap()
    }

    pub fn param_mut(&mut self, name: &str) -> &mut Param {
        self.params.iter_mut().find(|v| v.name() == name).unwrap()
    }

    pub fn return_type(&self) -> &crate::Type {
        &self.return_type
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.vis.is_private() {
            write!(f, "{} ", &self.vis)?;
        }

        if self.is_async {
            write!(f, "async ")?;
        }

        write!(f, "fn {}{}(", &self.name, &self.generics)?;

        for (i, param) in self.params.iter().enumerate() {
            write!(f, "{}", param)?;

            if i < self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")?;

        if !self.return_type.is_void() {
            write!(f, " -> {}", &self.return_type)?;
        }

        write!(f, ";")
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct MethodBuilder(crate::Method);

impl Default for MethodBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MethodBuilder {
    pub fn new() -> Self {
        Self(crate::Method {
            meta: crate::MetaData::new(),
            is_async: false,
            vis: crate::Visibility::Private,
            name: String::from(""),
            generics: crate::Generics::new(),
            params: vec![],
            return_type: Box::new(crate::Type::Void),
        })
    }

    pub fn with_name(&self, name: &str) -> Self {
        let mut next = self.clone();
        next.0.name = name.to_string();
        next
    }

    pub fn with_meta(&self, meta: &crate::MetaData) -> Self {
        let mut next = self.clone();
        next.0.meta = meta.clone();
        next
    }

    pub fn with_is_async(&self, is_async: bool) -> Self {
        let mut next = self.clone();
        next.0.is_async = is_async;
        next
    }

    pub fn with_visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
        next
    }

    pub fn with_generics(&self, generics: &crate::Generics) -> Self {
        let mut next = self.clone();
        next.0.generics = generics.clone();
        next
    }

    pub fn with_params(&self, params: &[crate::Param]) -> Self {
        let mut next = self.clone();
        next.0.params.append(&mut params.to_vec());
        next
    }

    pub fn with_param(&self, param: &crate::Param) -> Self {
        let mut next = self.clone();
        next.0.params.push(param.clone());
        next
    }

    pub fn with_return_type(&self, ty: &crate::Type) -> Self {
        let mut next = self.clone();
        next.0.return_type = Box::new(ty.clone());
        next
    }

    pub fn build(&self) -> crate::Method {
        self.0.clone()
    }
}
