#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModType {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) items: Vec<crate::Item>,
}

impl ModType {
    pub fn new() -> crate::ModTypeBuilder {
        return crate::ModTypeBuilder::new();
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Mod(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(self.path.to_string());
    }

    pub fn len(&self) -> usize {
        return self.items.len();
    }

    pub fn path(&self) -> &crate::Path {
        return &self.path;
    }

    pub fn meta(&self) -> &crate::MetaData {
        return &self.meta;
    }

    pub fn vis(&self) -> &crate::Visibility {
        return &self.vis;
    }

    pub fn items(&self) -> &[crate::Item] {
        return &self.items;
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_mod();
    }
}

impl crate::ToType for ModType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Mod(self.clone());
    }
}

impl std::ops::Index<usize> for ModType {
    type Output = crate::Item;

    fn index(&self, index: usize) -> &Self::Output {
        return self.items.index(index);
    }
}

impl std::ops::IndexMut<usize> for ModType {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.items.index_mut(index);
    }
}

impl std::fmt::Display for ModType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.vis.is_private() {
            write!(f, "{} ", &self.vis)?;
        }

        write!(f, "{} {{", self.path.name())?;

        for item in &self.items {
            write!(f, "\n\t{}", item)?;
        }

        if self.items.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct ModTypeBuilder(crate::ModType);

impl ModTypeBuilder {
    pub fn new() -> Self {
        return Self(crate::ModType {
            path: crate::Path::new(),
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            items: vec![],
        });
    }

    pub fn with_path(&self, path: &crate::Path) -> Self {
        let mut next = self.clone();
        next.0.path = path.clone();
        return next;
    }

    pub fn with_meta(&self, meta: &crate::MetaData) -> Self {
        let mut next = self.clone();
        next.0.meta = meta.clone();
        return next;
    }

    pub fn with_visibility(&self, vis: crate::Visibility) -> Self {
        let mut next = self.clone();
        next.0.vis = vis;
        return next;
    }

    pub fn with_items(&self, items: &[crate::Item]) -> Self {
        let mut next = self.clone();
        next.0.items.append(&mut items.to_vec());
        return next;
    }

    pub fn build(&self) -> crate::ModType {
        return self.0.clone();
    }
}
