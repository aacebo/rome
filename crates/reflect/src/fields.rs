use crate::{Field, FieldName};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fields {
    pub(crate) layout: crate::Layout,
    pub(crate) fields: Vec<Field>,
}

impl Fields {
    pub fn new() -> crate::FieldsBuilder {
        return crate::FieldsBuilder::new();
    }

    pub fn layout(&self) -> &crate::Layout {
        return &self.layout;
    }

    pub fn len(&self) -> usize {
        return self.fields.len();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Field> {
        return self.fields.iter();
    }

    pub fn has(&self, name: &FieldName) -> bool {
        return self.fields.iter().any(|v| v.name() == name);
    }

    pub fn get(&self, name: &FieldName) -> Option<&Field> {
        return self.fields.iter().find(|v| v.name() == name);
    }

    pub fn get_mut(&mut self, name: &FieldName) -> Option<&mut Field> {
        return self.fields.iter_mut().find(|v| v.name() == name);
    }
}

impl From<&[crate::Field]> for Fields {
    fn from(value: &[crate::Field]) -> Self {
        return Self::new().with_fields(value).build();
    }
}

impl<const N: usize> From<&[crate::Field; N]> for Fields {
    fn from(value: &[crate::Field; N]) -> Self {
        return Self::new().with_fields(value).build();
    }
}

impl<const N: usize> From<[crate::Field; N]> for Fields {
    fn from(value: [crate::Field; N]) -> Self {
        return Self::new().with_fields(&value).build();
    }
}

impl AsRef<Fields> for Fields {
    fn as_ref(&self) -> &Self {
        return self;
    }
}

impl AsMut<Fields> for Fields {
    fn as_mut(&mut self) -> &mut Self {
        return self;
    }
}

impl std::ops::Index<usize> for Fields {
    type Output = Field;

    fn index(&self, index: usize) -> &Self::Output {
        return self.get(&FieldName::from(index)).unwrap();
    }
}

impl std::ops::IndexMut<usize> for Fields {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.get_mut(&FieldName::from(index)).unwrap();
    }
}

impl std::ops::Index<&str> for Fields {
    type Output = Field;

    fn index(&self, index: &str) -> &Self::Output {
        return self.get(&FieldName::from(index)).unwrap();
    }
}

impl std::ops::IndexMut<&str> for Fields {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return self.get_mut(&FieldName::from(index)).unwrap();
    }
}

impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.layout.is_key() {
            write!(f, " {{")?;

            for field in &self.fields {
                write!(f, "\n\t{},", field)?;
            }

            if self.fields.len() > 0 {
                write!(f, "\n")?;
            }

            return write!(f, "}}");
        }

        if self.layout.is_index() {
            write!(f, "(")?;

            for (i, field) in self.fields.iter().enumerate() {
                if !field.vis.is_private() {
                    write!(f, "{} ", field.vis())?;
                }

                write!(f, "{}", field.ty())?;

                if i < self.fields.len() - 1 {
                    write!(f, ", ")?;
                }
            }

            return write!(f, ")");
        }

        return Ok(());
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct FieldsBuilder(crate::Fields);

impl FieldsBuilder {
    pub fn new() -> Self {
        return Self(crate::Fields {
            layout: crate::Layout::Unit,
            fields: vec![],
        });
    }

    pub fn with_layout(&self, layout: crate::Layout) -> Self {
        let mut next = self.clone();
        next.0.layout = layout;
        return next;
    }

    pub fn with_fields(&self, fields: &[crate::Field]) -> Self {
        let mut next = self.clone();
        next.0.fields.append(&mut fields.to_vec());
        return next;
    }

    pub fn with_field(&self, field: &crate::Field) -> Self {
        let mut next = self.clone();
        next.0.fields.push(field.clone());
        return next;
    }

    pub fn build(&self) -> crate::Fields {
        return self.0.clone();
    }
}
