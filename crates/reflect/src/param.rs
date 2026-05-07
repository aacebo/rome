#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Param {
    pub(crate) name: String,
    pub(crate) ty: Box<crate::Type>,
}

impl Param {
    pub fn new(name: &str, ty: &crate::Type) -> Self {
        return Self {
            name: name.to_string(),
            ty: Box::new(ty.clone()),
        };
    }

    pub fn is_selfish(&self) -> bool {
        return self.name == "self"
            && (self.ty.is_self()
                || self.ty.is_mut_self()
                || self.ty.is_ref_self()
                || self.ty.is_ref_mut_self());
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty;
    }
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_selfish() {
            let ty = self.ty.to_string();
            return write!(f, "{}self", &ty[0..ty.len() - 4]);
        }

        return write!(f, "{}: {}", &self.name, &self.ty);
    }
}
