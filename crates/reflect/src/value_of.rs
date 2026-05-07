#[macro_export]
macro_rules! value_of {
    [&mut $value:expr] => {
        $crate::AsValueMut::as_value(&mut $value)
    };
    [&mut $value:ident] => {
        $crate::AsValueMut::as_value(&mut $value)
    };
    [&mut $value:literal] => {
        $crate::AsValueMut::as_value(&mut $value)
    };
    [&$value:expr] => {
        $crate::ToValue::to_value(&$value)
    };
    [&$value:ident] => {
        $crate::ToValue::to_value(&$value)
    };
    [&$value:literal] => {
        $crate::ToValue::to_value(&$value)
    };
    [$value:expr] => {
        $crate::ToValue::to_value($value)
    };
    [$value:ident] => {
        $crate::ToValue::to_value($value)
    };
    [$value:literal] => {
        $crate::ToValue::to_value($value)
    };
    ($($anything:tt)*) => {
        $crate::value_of!($($anything)*)
    };
}

/// ## ToValue
///
/// implemented by types that
/// can reflect their value
pub trait ToValue {
    fn to_value(self) -> crate::Value;
}

/// ## AsValue
///
/// implemented by types that
/// can reflect their value
pub trait AsValue {
    fn as_value(&self) -> crate::Value;
}

/// ## AsValueMut
///
/// implemented by types that
/// can reflect their value
pub trait AsValueMut {
    fn as_value(&mut self) -> crate::Value;
}

#[cfg(test)]
mod test {
    #[test]
    pub fn from_expr() {
        let value = value_of!(1_i8);

        assert!(value.is_i8());
        assert_eq!(value.to_i8(), 1);
    }

    #[test]
    pub fn from_expr_ref() {
        let value = value_of!(&1_i8);

        assert!(value.is_ref());
        assert_eq!(value.to_ref().to_i8(), 1);
        assert_eq!(value.to_type().id(), "&i8");
    }
}
