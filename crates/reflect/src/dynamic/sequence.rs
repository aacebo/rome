use crate::TypeOf;

/// ## Sequence
///
/// implemented by types that can reflect their value/type
/// and the values of their individual elements
pub trait Sequence: std::fmt::Debug + crate::ToType {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn index(&self, i: usize) -> crate::Value<'_>;
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

impl<T> crate::TypeOf for Vec<T>
where
    T: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        crate::StructType::new()
            .path(crate::Path::from("std::vec"))
            .name("Vec")
            .visibility(crate::Visibility::Public(crate::Public::Full))
            .generics(crate::Generics::from([crate::TypeParam::new()
                .name("T")
                .build()
                .to_generic()]))
            .build()
            .to_type()
    }
}

impl<T> crate::ToType for Vec<T>
where
    T: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        Vec::<T>::type_of()
    }
}

impl<T> crate::ToValue for Vec<T>
where
    T: Clone + crate::TypeOf + crate::ToValue,
{
    fn to_value<'a>(&'a self) -> crate::Value<'a> {
        crate::Value::Slice(crate::Slice {
            ty: crate::SliceType {
                ty: std::rc::Rc::new(T::type_of()),
                capacity: None,
            },
            value: self.iter().map(|v| v.to_value()).collect(),
        })
    }
}

impl<T> crate::Sequence for Vec<T>
where
    T: Clone + std::fmt::Debug + crate::TypeOf + crate::ToValue + 'static,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn index(&self, i: usize) -> crate::Value<'_> {
        match self.get(i) {
            None => crate::Value::Null,
            Some(v) => v.to_value(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Dynamic;

    #[test]
    pub fn vec_sequence_index_returns_element() {
        let vec = vec![10_i32, 20, 30];
        let dynamic = Dynamic::from_sequence(&vec);

        assert_eq!(dynamic.len(), 3);
        assert_eq!(dynamic.as_sequence().index(1).to_i32(), 20);
    }
}
