use crate::TypeOf;

impl<T, E> crate::TypeOf for Result<T, E> {
    fn type_of() -> crate::Type {
        return crate::EnumType::new()
            .with_path(&crate::Path::from("core::result"))
            .with_name("Result")
            .with_visibility(crate::Visibility::Public(crate::Public::Full))
            .with_generics(&crate::Generics::from([
                crate::TypeParam::new().with_name("T").build().to_generic(),
                crate::TypeParam::new().with_name("E").build().to_generic(),
            ]))
            .build()
            .to_type();
    }
}

impl<T, E> crate::ToType for Result<T, E> {
    fn to_type(&self) -> crate::Type {
        return Result::<T, E>::type_of();
    }
}

impl<T: crate::ToValue, E: crate::ToValue> crate::ToValue for Result<T, E> {
    fn to_value(self) -> crate::Value {
        return match self {
            Err(err) => err.to_value(),
            Ok(v) => v.to_value(),
        };
    }
}

impl<T: crate::AsValue, E: crate::AsValue> crate::AsValue for Result<T, E> {
    fn as_value(&self) -> crate::Value {
        return match self {
            Err(err) => err.as_value(),
            Ok(v) => v.as_value(),
        };
    }
}
