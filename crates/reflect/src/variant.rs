#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    pub(crate) meta: crate::MetaData,
    pub(crate) name: String,
    pub(crate) fields: crate::Fields,
}

impl Variant {
    pub fn new() -> crate::VariantBuilder {
        return crate::VariantBuilder::new();
    }

    pub fn meta(&self) -> &crate::MetaData {
        return &self.meta;
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn fields(&self) -> &crate::Fields {
        return &self.fields;
    }

    pub fn len(&self) -> usize {
        return self.fields.len();
    }
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}{}", &self.name, &self.fields);
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct VariantBuilder(crate::Variant);

impl VariantBuilder {
    pub fn new() -> Self {
        return Self(crate::Variant {
            meta: crate::MetaData::new(),
            name: String::from(""),
            fields: crate::FieldsBuilder::new().build(),
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

    pub fn with_fields(&self, fields: &crate::Fields) -> Self {
        let mut next = self.clone();
        next.0.fields = fields.clone();
        return next;
    }

    pub fn build(&self) -> crate::Variant {
        return self.0.clone();
    }
}
