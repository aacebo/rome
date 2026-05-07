use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ref {
    pub(crate) ty: crate::RefType,
    pub(crate) value: Box<crate::Value>,
}

impl Ref {
    pub fn to_type(&self) -> crate::Type {
        return self.ty.to_type();
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty.0;
    }

    pub fn value(&self) -> &crate::Value {
        return &self.value;
    }
}

impl crate::ToValue for Ref {
    fn to_value(self) -> crate::Value {
        return crate::Value::Ref(self.clone());
    }
}

impl crate::AsValue for Ref {
    fn as_value(&self) -> crate::Value {
        return crate::Value::Ref(self.clone());
    }
}

impl AsRef<crate::Value> for Ref {
    fn as_ref(&self) -> &crate::Value {
        return &self.value;
    }
}

impl AsMut<crate::Value> for Ref {
    fn as_mut(&mut self) -> &mut crate::Value {
        return &mut self.value;
    }
}

impl std::ops::Deref for Ref {
    type Target = crate::Value;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

impl std::ops::DerefMut for Ref {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.value;
    }
}

impl std::fmt::Display for Ref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.value);
    }
}

impl PartialEq<crate::Value> for Ref {
    fn eq(&self, other: &crate::Value) -> bool {
        return other.is_ref() && other.to_ref() == *self;
    }
}

impl<T> crate::ToValue for &T
where
    T: Clone + crate::ToValue + crate::ToType,
{
    fn to_value(self) -> crate::Value {
        return crate::Value::Ref(Ref {
            ty: crate::RefType(Box::new(self.clone().to_type())),
            value: Box::new(self.clone().to_value()),
        });
    }
}

impl<T> crate::AsValue for &T
where
    T: Clone + crate::AsValue + crate::ToType,
{
    #[allow(suspicious_double_ref_op)]
    fn as_value(&self) -> crate::Value {
        let value = self.clone().clone();

        return crate::Value::Ref(Ref {
            ty: crate::RefType(Box::new(value.to_type())),
            value: Box::new(value.as_value()),
        });
    }
}

impl<T> crate::ToValue for Arc<T>
where
    T: Clone + crate::ToValue + crate::ToType,
{
    fn to_value(self) -> crate::Value {
        return crate::Value::Ref(Ref {
            ty: crate::RefType(Box::new(self.as_ref().to_type())),
            value: Box::new(self.as_ref().clone().to_value()),
        });
    }
}

impl<T> crate::AsValue for Arc<T>
where
    T: Clone + crate::AsValue + crate::ToType,
{
    fn as_value(&self) -> crate::Value {
        return crate::Value::Ref(Ref {
            ty: crate::RefType(Box::new(self.as_ref().to_type())),
            value: Box::new(self.as_ref().clone().as_value()),
        });
    }
}

impl<T> From<&T> for crate::Value
where
    T: Clone + crate::ToValue + crate::ToType,
{
    fn from(value: &T) -> Self {
        return Self::Ref(Ref {
            ty: crate::RefType(Box::new(value.to_type())),
            value: Box::new(value.clone().to_value()),
        });
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
