#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mut {
    pub(crate) ty: crate::MutType,
    pub(crate) value: std::rc::Rc<crate::Value>,
}

impl Mut {
    pub fn to_type(&self) -> crate::Type {
        self.ty.to_type()
    }

    pub fn ty(&self) -> &crate::Type {
        &self.ty.0
    }

    pub fn value(&self) -> &crate::Value {
        &self.value
    }
}

impl<T> crate::ToValue for &mut T
where
    T: Clone + crate::ToType + crate::ToValue,
{
    fn to_value(self) -> crate::Value {
        let value = self.clone();

        crate::Value::Mut(Mut {
            ty: crate::MutType::new(value.to_type()),
            value: std::rc::Rc::new(value.to_value()),
        })
    }
}

impl<T> crate::AsValueMut for T
where
    T: Clone + crate::ToType + crate::ToValue + crate::AsValue,
{
    fn as_value(&mut self) -> crate::Value {
        let value = self.clone();

        crate::Value::Mut(Mut {
            ty: crate::MutType::new(value.to_type()),
            value: std::rc::Rc::new(value.as_value()),
        })
    }
}

impl<T: Clone + crate::ToType + crate::ToValue> From<&T> for Mut {
    fn from(value: &T) -> Self {
        Self {
            ty: crate::MutType::new(value.to_type()),
            value: std::rc::Rc::new(value.clone().to_value()),
        }
    }
}

impl std::fmt::Display for Mut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl crate::ToType for Mut {
    fn to_type(&self) -> crate::Type {
        self.ty.to_type()
    }
}

impl crate::ToValue for Mut {
    fn to_value(self) -> crate::Value {
        crate::Value::Mut(self.clone())
    }
}

impl crate::AsValue for Mut {
    fn as_value(&self) -> crate::Value {
        crate::Value::Mut(self.clone())
    }
}

impl AsRef<crate::Value> for Mut {
    fn as_ref(&self) -> &crate::Value {
        &self.value
    }
}

impl AsMut<crate::Value> for Mut {
    fn as_mut(&mut self) -> &mut crate::Value {
        std::rc::Rc::make_mut(&mut self.value)
    }
}

impl std::ops::Deref for Mut {
    type Target = crate::Value;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for Mut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        std::rc::Rc::make_mut(&mut self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::value_of;

    #[test]
    pub fn basic() {
        let value = value_of!(&mut 11);
        assert!(value.is_mut());
    }
}
