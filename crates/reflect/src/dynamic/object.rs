use std::sync::Arc;

/// ## Object
///
/// implemented by types that
/// can reflect their value/type and that
/// of their individual fields
pub trait Object: crate::Dyn {
    fn field(&self, name: &crate::FieldName) -> crate::Value;
}

impl dyn Object {
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
impl serde::Serialize for dyn Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let ty = self.to_type().to_struct();
        let mut ser = serializer.serialize_map(Some(ty.len()))?;

        for field in ty.fields().iter() {
            ser.serialize_entry(&field.name().to_string(), &self.field(field.name()))?;
        }

        return ser.end();
    }
}

impl std::fmt::Display for dyn Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.to_type().to_struct();
        write!(f, "{{")?;

        for field in ty.fields().iter() {
            write!(f, "\n\t{}: {}", field.name(), self.field(field.name()))?;
        }

        return write!(f, "\n}}");
    }
}

impl<T: Clone + Object> Object for Arc<T> {
    fn field(&self, name: &crate::FieldName) -> crate::Value {
        return self.as_ref().field(name);
    }
}
