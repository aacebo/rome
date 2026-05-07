#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrType;

impl StrType {
    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Str(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_str("string");
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_str();
    }
}

impl std::fmt::Display for StrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.id());
    }
}

impl crate::ToType for StrType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Str(self.clone());
    }
}

impl crate::TypeOf for std::string::String {
    fn type_of() -> crate::Type {
        return crate::Type::Str(StrType::default());
    }
}

impl crate::ToType for std::string::String {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Str(StrType::default());
    }
}

impl crate::TypeOf for str {
    fn type_of() -> crate::Type {
        return crate::Type::Str(StrType::default());
    }
}

impl crate::ToType for str {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Str(StrType::default());
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn str() {
        let value = value_of!("test");

        assert!(value.is_str());
        assert_eq!(value.len(), 4);
        assert_eq!(value.to_string(), "test");
    }

    #[test]
    pub fn string() {
        let value = value_of!("test".to_string());

        assert!(value.is_str());
        assert_eq!(value.len(), 4);
        assert_eq!(value.to_string(), "test");
    }
}
