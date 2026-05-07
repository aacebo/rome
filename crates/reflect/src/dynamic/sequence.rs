use std::sync::Arc;

use crate::TypeOf;

/// ## Sequence
///
/// implemented by types that
/// can reflect their value/type and that
/// of their individual index's in a sequence
pub trait Sequence: crate::Dyn {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn index(&self, i: usize) -> crate::Value;
    fn index_ref(&self, _: usize) -> &crate::Value {
        unimplemented!()
    }
}

impl dyn Sequence {
    pub fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
        let value: &dyn std::any::Any = self;
        value.downcast_ref::<T>()
    }

    pub fn downcast_mut<T: std::any::Any>(&mut self) -> Option<&mut T> {
        let value: &mut dyn std::any::Any = self;
        value.downcast_mut::<T>()
    }

    pub fn is<T: std::any::Any>(&self) -> bool {
        let value: &dyn std::any::Any = self;
        value.is::<T>()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn Sequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let ty = self.to_type().to_slice();
        let mut ser = serializer.serialize_seq(ty.capacity())?;

        for i in 0..self.len() {
            ser.serialize_element(&self.index(i))?;
        }

        ser.end()
    }
}

impl std::fmt::Display for dyn Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for i in 0..self.len() {
            write!(f, "\n\t{}", self.index(i))?;
        }

        write!(f, "\n]")
    }
}

impl<T: Clone + Sequence> Sequence for Arc<T> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn index(&self, i: usize) -> crate::Value {
        self.as_ref().index(i)
    }
}

impl<T> crate::TypeOf for Vec<T> {
    fn type_of() -> crate::Type {
        crate::StructType::new()
            .with_path(&crate::Path::from("std::vec"))
            .with_name("Vec")
            .with_visibility(crate::Visibility::Public(crate::Public::Full))
            .with_generics(&crate::Generics::from([crate::TypeParam::new()
                .with_name("T")
                .build()
                .to_generic()]))
            .build()
            .to_type()
    }
}

impl<T> crate::ToType for Vec<T> {
    fn to_type(&self) -> crate::Type {
        Vec::<T>::type_of()
    }
}

impl<T> crate::ToValue for Vec<T>
where
    T: Clone + crate::TypeOf + crate::AsValue,
{
    fn to_value(self) -> crate::Value {
        crate::Value::Slice(crate::Slice {
            ty: crate::SliceType {
                ty: Box::new(T::type_of()),
                capacity: None,
            },
            value: self.iter().map(|v| v.as_value()).collect::<Vec<_>>(),
        })
    }
}

impl<T> crate::AsValue for Vec<T>
where
    T: Clone + crate::TypeOf + crate::AsValue,
{
    fn as_value(&self) -> crate::Value {
        crate::Value::Slice(crate::Slice {
            ty: crate::SliceType {
                ty: Box::new(T::type_of()),
                capacity: None,
            },
            value: self.iter().map(|v| v.as_value()).collect::<Vec<_>>(),
        })
    }
}

impl<T> crate::Sequence for Vec<T>
where
    T: Clone + std::fmt::Debug + crate::TypeOf + crate::AsValue + 'static,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn index(&self, i: usize) -> crate::Value {
        match self.get(i) {
            None => crate::Value::Null,
            Some(v) => v.as_value(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Dynamic;

    #[test]
    pub fn vec_sequence_index_returns_element() {
        let dynamic = Dynamic::from_sequence(vec![10_i32, 20, 30]);

        assert_eq!(dynamic.len(), 3);
        assert_eq!(dynamic.as_sequence().index(1).to_i32(), 20);
    }
}
