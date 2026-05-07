#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    pub(crate) meta: crate::MetaData,
    pub(crate) name: String,
    pub(crate) fields: crate::Fields,
}

impl Variant {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> crate::VariantBuilder {
        crate::VariantBuilder::new()
    }

    pub fn meta(&self) -> &crate::MetaData {
        &self.meta
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &crate::Fields {
        &self.fields
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", &self.name, &self.fields)
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct VariantBuilder(crate::Variant);

impl Default for VariantBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VariantBuilder {
    pub fn new() -> Self {
        Self(crate::Variant {
            meta: crate::MetaData::new(),
            name: String::from(""),
            fields: crate::FieldsBuilder::new().build(),
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

    pub fn with_fields(&self, fields: &crate::Fields) -> Self {
        let mut next = self.clone();
        next.0.fields = fields.clone();
        next
    }

    pub fn build(&self) -> crate::Variant {
        self.0.clone()
    }
}
