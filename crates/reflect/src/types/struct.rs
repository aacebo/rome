#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructType {
    pub(crate) path: crate::Path,
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) name: String,
    pub(crate) generics: crate::Generics,
    pub(crate) fields: crate::Fields,
}

impl StructType {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> crate::StructTypeBuilder {
        crate::StructTypeBuilder::new()
    }

    pub fn to_type(&self) -> crate::Type {
        crate::Type::Struct(self.clone())
    }

    pub fn id(&self) -> crate::TypeId {
        crate::TypeId::from_string(format!("{}::{}", &self.path, &self.name))
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
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

    pub fn fields(&self) -> &crate::Fields {
        &self.fields
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        self.id() == ty.id()
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        ty.is_struct()
    }
}

impl crate::ToType for StructType {
    fn to_type(&self) -> crate::Type {
        crate::Type::Struct(self.clone())
    }
}

impl std::fmt::Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vis != crate::Visibility::Private {
            write!(f, "{} ", &self.vis)?;
        }

        write!(f, "struct {}{}{}", &self.name, &self.generics, &self.fields)
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct StructTypeBuilder(crate::StructType);

impl Default for StructTypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StructTypeBuilder {
    pub fn new() -> Self {
        Self(crate::StructType {
            path: crate::Path::new(),
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            name: String::from(""),
            generics: crate::Generics::new(),
            fields: crate::FieldsBuilder::new().build(),
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

    pub fn with_fields(&self, fields: &crate::Fields) -> Self {
        let mut next = self.clone();
        next.0.fields = fields.clone();
        next
    }

    pub fn build(&self) -> crate::StructType {
        self.0.clone()
    }
}
