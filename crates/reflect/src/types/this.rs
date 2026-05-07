#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThisType;

impl ThisType {
    pub fn to_type(&self) -> crate::Type {
        return crate::Type::This(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_str("Self");
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_self();
    }
}

impl crate::ToType for ThisType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::This(self.clone());
    }
}

impl std::fmt::Display for ThisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.id());
    }
}
