use std::sync::Arc;

/// ## Dyn
///
/// implemented by types that
/// can reflect their value/type
pub trait Dyn:
    std::any::Any + std::fmt::Debug + crate::ToType + crate::ToValue + crate::AsValue
{
}

impl dyn Dyn {
    pub fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
        let value: &dyn std::any::Any = self;
        return value.downcast_ref::<T>();
    }

    pub fn downcast_mut<T: std::any::Any>(&mut self) -> Option<&mut T> {
        let value: &mut dyn std::any::Any = self;
        return value.downcast_mut::<T>();
    }

    pub fn is<T: std::any::Any>(&self) -> bool {
        let value: &dyn std::any::Any = self;
        return value.is::<T>();
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn Dyn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return self.as_value().serialize(serializer);
    }
}

impl std::fmt::Display for dyn Dyn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.as_value());
    }
}

impl<T: Clone + Dyn> Dyn for Arc<T> {}
impl<T> Dyn for Vec<T> where T: Clone + std::fmt::Debug + crate::TypeOf + crate::AsValue + 'static {}
