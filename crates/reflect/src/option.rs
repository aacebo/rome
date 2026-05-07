use crate::TypeOf;

impl<T> crate::TypeOf for Option<T> {
    fn type_of() -> crate::Type {
        return crate::EnumType::new()
            .with_path(&crate::Path::from("core::option"))
            .with_name("Option")
            .with_visibility(crate::Visibility::Public(crate::Public::Full))
            .with_generics(&crate::Generics::from([crate::TypeParam::new()
                .with_name("T")
                .build()
                .to_generic()]))
            .build()
            .to_type();
    }
}

impl<T> crate::ToType for Option<T> {
    fn to_type(&self) -> crate::Type {
        return Option::<T>::type_of();
    }
}

impl<T: crate::ToValue> crate::ToValue for Option<T> {
    fn to_value(self) -> crate::Value {
        return match self {
            None => crate::Value::Null,
            Some(v) => v.to_value(),
        };
    }
}

impl<T: crate::AsValue> crate::AsValue for Option<T> {
    fn as_value(&self) -> crate::Value {
        return match self {
            None => crate::Value::Null,
            Some(v) => v.as_value(),
        };
    }
}
