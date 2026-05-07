#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumType {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) name: String,
    pub(crate) generics: crate::Generics,
    pub(crate) variants: Vec<crate::Variant>,
}

impl EnumType {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> EnumTypeBuilder {
        EnumTypeBuilder::new()
    }

    pub fn to_type(&self) -> crate::Type {
        crate::Type::Enum(self.clone())
    }

    pub fn id(&self) -> crate::TypeId {
        crate::TypeId::from_string(format!("{}::{}", &self.path, &self.name))
    }

    pub fn len(&self) -> usize {
        self.variants.len()
    }

    pub fn is_empty(&self) -> bool {
        self.variants.is_empty()
    }

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

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        self.id() == ty.id()
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        ty.is_enum()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Variant> {
        self.variants.iter()
    }

    pub fn has_variant(&self, name: &str) -> bool {
        self.variants.iter().any(|v| v.name() == name)
    }

    pub fn variant(&self, name: &str) -> &crate::Variant {
        self.variants.iter().find(|v| v.name() == name).unwrap()
    }

    pub fn variant_mut(&mut self, name: &str) -> &mut crate::Variant {
        self.variants.iter_mut().find(|v| v.name() == name).unwrap()
    }
}

impl crate::ToType for EnumType {
    fn to_type(&self) -> crate::Type {
        crate::Type::Enum(self.clone())
    }
}

impl std::fmt::Display for EnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vis != crate::Visibility::Private {
            write!(f, "{} ", &self.vis)?;
        }

        write!(f, "enum {}{} {{", &self.name, &self.generics)?;

        for variant in &self.variants {
            write!(f, "\n\t{},", variant)?;
        }

        if !self.variants.is_empty() {
            writeln!(f)?;
        }

        write!(f, "}}")
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct EnumTypeBuilder(crate::EnumType);

impl Default for EnumTypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EnumTypeBuilder {
    pub fn new() -> Self {
        Self(crate::EnumType {
            path: crate::Path::new(),
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            name: String::new(),
            generics: crate::Generics::new(),
            variants: vec![],
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

    pub fn with_variants(&self, variants: &[crate::Variant]) -> Self {
        let mut next = self.clone();
        next.0.variants.append(&mut variants.to_vec());
        next
    }

    pub fn with_variant(&self, variant: &crate::Variant) -> Self {
        let mut next = self.clone();
        next.0.variants.push(variant.clone());
        next
    }

    pub fn build(&self) -> crate::EnumType {
        self.0.clone()
    }
}
