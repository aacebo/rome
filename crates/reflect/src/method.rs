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
    pub fn new() -> crate::MethodBuilder {
        return crate::MethodBuilder::new();
    }

    pub fn meta(&self) -> &crate::MetaData {
        return &self.meta;
    }

    pub fn is_async(&self) -> bool {
        return self.is_async;
    }

    pub fn vis(&self) -> Visibility {
        return self.vis.clone();
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn generics(&self) -> &crate::Generics {
        return &self.generics;
    }

    pub fn params(&self) -> &[Param] {
        return &self.params;
    }

    pub fn has_param(&self, name: &str) -> bool {
        return self.params.iter().any(|v| v.name() == name);
    }

    pub fn param(&self, name: &str) -> &Param {
        return self.params.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn param_mut(&mut self, name: &str) -> &mut Param {
        return self.params.iter_mut().find(|v| v.name() == name).unwrap();
    }

    pub fn return_type(&self) -> &crate::Type {
        return &self.return_type;
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

        return write!(f, ";");
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct MethodBuilder(crate::Method);

impl MethodBuilder {
    pub fn new() -> Self {
        return Self(crate::Method {
            meta: crate::MetaData::new(),
            is_async: false,
            vis: crate::Visibility::Private,
            name: String::from(""),
            generics: crate::Generics::new(),
            params: vec![],
            return_type: Box::new(crate::Type::Void),
        });
    }

    pub fn with_name(&self, name: &str) -> Self {
        let mut next = self.clone();
        next.0.name = name.to_string();
        return next;
    }

    pub fn with_meta(&self, meta: &crate::MetaData) -> Self {
        let mut next = self.clone();
        next.0.meta = meta.clone();
        return next;
    }

    pub fn with_is_async(&self, is_async: bool) -> Self {
        let mut next = self.clone();
        next.0.is_async = is_async;
        return next;
    }

    pub fn with_visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
        return next;
    }

    pub fn with_generics(&self, generics: &crate::Generics) -> Self {
        let mut next = self.clone();
        next.0.generics = generics.clone();
        return next;
    }

    pub fn with_params(&self, params: &[crate::Param]) -> Self {
        let mut next = self.clone();
        next.0.params.append(&mut params.to_vec());
        return next;
    }

    pub fn with_param(&self, param: &crate::Param) -> Self {
        let mut next = self.clone();
        next.0.params.push(param.clone());
        return next;
    }

    pub fn with_return_type(&self, ty: &crate::Type) -> Self {
        let mut next = self.clone();
        next.0.return_type = Box::new(ty.clone());
        return next;
    }

    pub fn build(&self) -> crate::Method {
        return self.0.clone();
    }
}
