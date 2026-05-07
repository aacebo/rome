#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    pub(crate) meta: crate::MetaData,
    pub(crate) vis: crate::Visibility,
    pub(crate) name: FieldName,
    pub(crate) ty: Box<crate::Type>,
}

impl Field {
    pub fn new() -> crate::FieldBuilder {
        return crate::FieldBuilder::new();
    }

    pub fn meta(&self) -> &crate::MetaData {
        return &self.meta;
    }

    pub fn vis(&self) -> &crate::Visibility {
        return &self.vis;
    }

    pub fn name(&self) -> &FieldName {
        return &self.name;
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty;
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.vis.is_private() {
            write!(f, "{} ", &self.vis)?;
        }

        return write!(f, "{}: {},", &self.name, &self.ty);
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FieldName {
    Key(String),
    Index(usize),
}

impl FieldName {
    pub fn is_key(&self) -> bool {
        return match self {
            Self::Key(_) => true,
            _ => false,
        };
    }

    pub fn is_index(&self) -> bool {
        return match self {
            Self::Index(_) => true,
            _ => false,
        };
    }

    pub fn as_str(&self) -> &str {
        return match self {
            Self::Key(v) => &v,
            _ => panic!("called 'as_str' on 'FieldName::Index'"),
        };
    }

    pub fn as_index(&self) -> &usize {
        return match self {
            Self::Index(v) => v,
            _ => panic!("called 'as_index' on 'FieldName::Key'"),
        };
    }
}

impl AsRef<Self> for FieldName {
    fn as_ref(&self) -> &Self {
        return self;
    }
}

impl AsMut<Self> for FieldName {
    fn as_mut(&mut self) -> &mut Self {
        return self;
    }
}

impl From<&FieldName> for FieldName {
    fn from(value: &FieldName) -> Self {
        return value.clone();
    }
}

impl From<&str> for FieldName {
    fn from(value: &str) -> Self {
        return Self::Key(value.to_string());
    }
}

impl From<String> for FieldName {
    fn from(value: String) -> Self {
        return Self::Key(value);
    }
}

impl From<&usize> for FieldName {
    fn from(value: &usize) -> Self {
        return Self::Index(value.clone());
    }
}

impl From<usize> for FieldName {
    fn from(value: usize) -> Self {
        return Self::Index(value);
    }
}

impl Eq for FieldName {}

impl Ord for FieldName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.partial_cmp(self).unwrap();
    }
}

impl PartialEq<str> for FieldName {
    fn eq(&self, other: &str) -> bool {
        return self.to_string() == other;
    }
}

impl PartialEq<String> for FieldName {
    fn eq(&self, other: &String) -> bool {
        return self.to_string() == other.as_str();
    }
}

impl PartialEq<usize> for FieldName {
    fn eq(&self, other: &usize) -> bool {
        return self.as_index() == other;
    }
}

impl std::fmt::Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Key(v) => write!(f, "{}", v),
            Self::Index(v) => write!(f, "{}", v),
        };
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct FieldBuilder(crate::Field);

impl FieldBuilder {
    pub fn new() -> Self {
        return Self(crate::Field {
            meta: crate::MetaData::new(),
            vis: crate::Visibility::Private,
            name: crate::FieldName::from(""),
            ty: Box::new(crate::Type::Any),
        });
    }

    pub fn with_name(&self, name: &crate::FieldName) -> Self {
        let mut next = self.clone();
        next.0.name = name.clone();
        return next;
    }

    pub fn with_type(&self, ty: &crate::Type) -> Self {
        let mut next = self.clone();
        next.0.ty = Box::new(ty.clone());
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

    pub fn build(&self) -> crate::Field {
        return self.0.clone();
    }
}
