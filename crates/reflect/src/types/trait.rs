#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TraitType {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) name: String,
    pub(crate) generics: crate::Generics,
    pub(crate) methods: Vec<crate::Method>,
}

impl TraitType {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> crate::TraitTypeBuilder {
        crate::TraitTypeBuilder::new()
    }

    pub fn to_type(&self) -> crate::Type {
        crate::Type::Trait(self.clone())
    }

    pub fn id(&self) -> crate::TypeId {
        crate::TypeId::from_string(format!("{}::{}", &self.path, &self.name))
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        self.id() == ty.id()
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        ty.is_trait()
    }
}

impl TraitType {
    pub fn path(&self) -> &crate::Path {
        &self.path
    }

    pub fn meta(&self) -> &crate::MetaData {
        &self.meta
    }

    pub fn vis(&self) -> &crate::Visibility {
        &self.vis
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn generics(&self) -> &crate::Generics {
        &self.generics
    }

    pub fn methods(&self) -> &[crate::Method] {
        &self.methods
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Method> {
        self.methods.iter()
    }

    pub fn len(&self) -> usize {
        self.methods.len()
    }

    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    }

    pub fn has(&self, name: &str) -> bool {
        self.methods.iter().any(|v| v.name() == name)
    }

    pub fn get(&self, name: &str) -> Option<&crate::Method> {
        self.methods.iter().find(|v| v.name() == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut crate::Method> {
        self.methods.iter_mut().find(|v| v.name() == name)
    }
}

impl crate::ToType for TraitType {
    fn to_type(&self) -> crate::Type {
        crate::Type::Trait(self.clone())
    }
}

impl AsRef<TraitType> for TraitType {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TraitType> for TraitType {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl std::ops::Index<usize> for TraitType {
    type Output = crate::Method;

    fn index(&self, index: usize) -> &Self::Output {
        self.methods.index(index)
    }
}

impl std::ops::IndexMut<usize> for TraitType {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.methods.index_mut(index)
    }
}

impl std::ops::Index<&str> for TraitType {
    type Output = crate::Method;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl std::ops::IndexMut<&str> for TraitType {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl std::fmt::Display for TraitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.vis.is_private() {
            write!(f, "{} ", &self.vis)?;
        }

        write!(f, "trait {}{} {{", &self.name, &self.generics)?;

        for method in &self.methods {
            write!(f, "\n\t{}", method)?;
        }

        if !self.methods.is_empty() {
            writeln!(f)?;
        }

        write!(f, "}}")
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct TraitTypeBuilder(crate::TraitType);

impl Default for TraitTypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TraitTypeBuilder {
    pub fn new() -> Self {
        Self(crate::TraitType {
            path: crate::Path::new(),
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            name: String::from(""),
            generics: crate::Generics::new(),
            methods: vec![],
        })
    }

    pub fn with_path(&self, path: &crate::Path) -> Self {
        let mut next = self.clone();
        next.0.path = path.clone();
        next
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

    pub fn with_methods(&self, methods: &[crate::Method]) -> Self {
        let mut next = self.clone();
        next.0.methods.append(&mut methods.to_vec());
        next
    }

    pub fn with_method(&self, method: &crate::Method) -> Self {
        let mut next = self.clone();
        next.0.methods.push(method.clone());
        next
    }

    pub fn build(&self) -> crate::TraitType {
        self.0.clone()
    }
}
