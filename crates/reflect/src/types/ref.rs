use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct RefType(pub(crate) Box<crate::Type>);

impl RefType {
    pub fn new(ty: &crate::Type) -> Self {
        return Self(Box::new(ty.clone()));
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Ref(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("&{}", self.0.id()));
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.0;
    }

    pub fn is_ref_of(&self, ty: crate::Type) -> bool {
        return ty.eq(&self.0);
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_ref_of(*self.0.clone());
    }
}

impl std::fmt::Display for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "&{}", &self.0);
    }
}

impl crate::ToType for RefType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Ref(self.clone());
    }
}

impl PartialEq<crate::Type> for RefType {
    fn eq(&self, other: &crate::Type) -> bool {
        return match other {
            crate::Type::Ref(v) => v == self,
            _ => false,
        };
    }
}

impl<T> crate::TypeOf for &T
where
    T: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return crate::RefType::new(&T::type_of()).to_type();
    }
}

impl<T> crate::ToType for &T
where
    T: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return crate::RefType::new(&T::type_of()).to_type();
    }
}

impl<T> crate::ToType for Arc<T>
where
    T: Clone + crate::ToType,
{
    fn to_type(&self) -> crate::Type {
        return crate::RefType::new(&self.as_ref().to_type()).to_type();
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn int() {
        let value = value_of!(&3_i32);

        assert!(value.is_ref());
        assert_eq!(value.to_ref().ty(), &type_of!(i32));
        assert_eq!(value.to_type().id(), "&i32");
        assert_eq!(value.to_i32(), 3);
    }

    #[test]
    pub fn bool() {
        let value = value_of!(&true);

        assert!(value.is_ref());
        assert_eq!(value.to_ref().ty(), &type_of!(bool));
        assert_eq!(value.to_type().id(), "&bool");
        assert!(value.to_bool());
    }
}
