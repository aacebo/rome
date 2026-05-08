#[derive(Debug, Clone, PartialEq)]
pub struct Slice<'a> {
    pub(crate) ty: crate::SliceType,
    pub(crate) value: Vec<crate::Value<'a>>,
}

impl<'a> Slice<'a> {
    pub fn to_type(&self) -> crate::Type {
        crate::Type::Slice(self.ty.clone())
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value<'a>> {
        self.value.iter()
    }
}

impl<'a> PartialEq<crate::Value<'a>> for Slice<'a> {
    fn eq(&self, other: &crate::Value<'a>) -> bool {
        other.is_slice() && other.as_slice() == self
    }
}

impl<'a> From<Vec<crate::Value<'a>>> for Slice<'a> {
    fn from(value: Vec<crate::Value<'a>>) -> Self {
        let ty = std::rc::Rc::new(
            value
                .first()
                .map(crate::ToType::to_type)
                .unwrap_or(crate::Type::Any),
        );
        Self {
            ty: crate::SliceType { ty, capacity: None },
            value,
        }
    }
}

impl<'a> std::fmt::Display for Slice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.value.iter().enumerate() {
            write!(f, "{}", value)?;
            if i < self.value.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<'a> crate::ToType for Slice<'a> {
    fn to_type(&self) -> crate::Type {
        crate::Type::Slice(self.ty.clone())
    }
}

impl<'a> crate::ToValue for Slice<'a> {
    fn to_value(&self) -> crate::Value<'_> {
        crate::Value::Slice(self.clone())
    }
}

impl<T> crate::ToValue for &[T]
where
    T: Clone + crate::TypeOf + crate::ToValue,
{
    fn to_value(&self) -> crate::Value<'_> {
        crate::Value::Slice(Slice {
            ty: crate::SliceType {
                ty: std::rc::Rc::new(T::type_of()),
                capacity: None,
            },
            value: self.iter().map(|v| v.to_value()).collect(),
        })
    }
}

impl<const N: usize, T> crate::ToValue for [T; N]
where
    T: Clone + crate::TypeOf + crate::ToValue,
{
    fn to_value(&self) -> crate::Value<'_> {
        crate::Value::Slice(Slice {
            ty: crate::SliceType {
                ty: std::rc::Rc::new(T::type_of()),
                capacity: Some(N),
            },
            value: self.iter().map(|v| v.to_value()).collect(),
        })
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for Slice<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(s)
    }
}

impl<'a> AsRef<[crate::Value<'a>]> for Slice<'a> {
    fn as_ref(&self) -> &[crate::Value<'a>] {
        self.value.as_slice()
    }
}

impl<'a> std::ops::Deref for Slice<'a> {
    type Target = [crate::Value<'a>];

    fn deref(&self) -> &Self::Target {
        self.value.as_slice()
    }
}

impl<'a> std::ops::Index<usize> for Slice<'a> {
    type Output = crate::Value<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        self.value.index(index)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn ok() {
        let value = value_of!([1, 2, 3]);

        assert!(value.is_slice());
        assert_eq!(value.len(), 3);
        assert_eq!(value.to_type().len(), 3);
        assert_eq!(value.to_type().id(), "[i32; 3]");

        for (i, value) in value.as_slice().iter().enumerate() {
            assert!(value.is_i32());
            assert_eq!(i + 1, value.to_i32() as usize);
        }
    }
}
