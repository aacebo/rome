#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct MutType(pub(crate) Box<crate::Type>);

impl MutType {
    pub fn new(ty: &crate::Type) -> Self {
        return Self(Box::new(ty.clone()));
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Mut(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_string(format!("mut {}", &self.0.id()));
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.0;
    }

    pub fn is_mut_of(&self, ty: crate::Type) -> bool {
        return ty.eq(&self.0);
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_mut();
    }
}

impl crate::ToType for MutType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Mut(self.clone());
    }
}

impl std::fmt::Display for MutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "mut {}", &self.0);
    }
}

impl<T> crate::TypeOf for &mut T
where
    T: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return crate::MutType::new(&T::type_of()).to_type();
    }
}

impl<T> crate::ToType for &mut T
where
    T: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return crate::MutType::new(&T::type_of()).to_type();
    }
}

#[cfg(test)]
mod test {
    use crate::{TypeOf, type_of};

    #[test]
    pub fn basic() {
        let ty = type_of!(&mut i8);
        assert!(ty.is_mut());
    }
}
