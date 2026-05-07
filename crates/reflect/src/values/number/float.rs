macro_rules! float {
    ($($name:ident $type_name:ident $is_type:ident $to_type:ident $set_value:ident $type:ty)*) => {
        ///
        /// Value: Implementations
        ///

        impl crate::Value {
            pub fn is_float(&self) -> bool {
                return match self {
                    Self::Number(v) => v.is_float(),
                    _ => false,
                };
            }

            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::Number(v) => v.$is_type(),
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $type {
                    return match self {
                        Self::Number(v) => v.$to_type(),
                        Self::Ref(v) => v.value().$to_type(),
                        Self::Mut(v) => v.value().$to_type(),
                        v => panic!("called '{}' on type '{}'", stringify!($to_type), v.to_type()),
                    };
                }
            )*
        }

        $(
            impl crate::ToValue for $type {
                fn to_value(self) -> crate::Value {
                    return crate::Value::Number(crate::Number::Float(crate::Float::$name(self)));
                }
            }

            impl crate::AsValue for $type {
                fn as_value(&self) -> crate::Value {
                    return crate::Value::Number(crate::Number::Float(crate::Float::$name(*self)));
                }
            }

            impl From<$type> for crate::Value {
                fn from(value: $type) -> Self {
                    return Self::Number(crate::Number::Float(crate::Float::$name(value)));
                }
            }

            impl Into<$type> for crate::Value {
                fn into(self) -> $type {
                    return match self {
                        Self::Number(v) => v.to_float().$to_type(),
                        v => panic!("called 'Into<{}>::into' on type '{}'", stringify!($type), v.to_type()),
                    };
                }
            }

            impl AsRef<$type> for crate::Value {
                fn as_ref(&self) -> &$type {
                    return match self {
                        Self::Number(v) => AsRef::<$type>::as_ref(v),
                        v => panic!("called 'AsRef<{}>::as_ref' on type '{}'", stringify!($type), v.to_type()),
                    };
                }
            }

            impl AsMut<$type> for crate::Value {
                fn as_mut(&mut self) -> &mut $type {
                    return match self {
                        Self::Number(v) => AsMut::<$type>::as_mut(v),
                        v => panic!("called 'AsMut<{}>::as_mut' on type '{}'", stringify!($type), v.to_type()),
                    };
                }
            }
        )*

        ///
        /// Number: Implementations
        ///

        impl crate::Number {
            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::Float(v) => v.$is_type(),
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $type {
                    return match self {
                        Self::Float(v) => v.$to_type(),
                        v => panic!("called '{}' on type '{}'", stringify!($to_type), v.to_type()),
                    };
                }
            )*
        }

        $(
            impl From<$type> for crate::Number {
                fn from(value: $type) -> Self {
                    return Self::Float(Float::$name(value));
                }
            }

            impl Into<$type> for crate::Number {
                fn into(self) -> $type {
                    return match self {
                        Self::Float(v) => v.$to_type(),
                        v => panic!("called 'Into<{}>::into' on '{}'", stringify!($type), v.to_type()),
                    };
                }
            }

            impl AsRef<$type> for crate::Number {
                fn as_ref(&self) -> &$type {
                    return match self {
                        Self::Float(v) => match v {
                            Float::$name(v) => v,
                            v => panic!("called 'AsRef<{}>::as_ref' on '{}'", stringify!($type), v.to_type()),
                        },
                        v => panic!("called 'AsRef<{}>::as_ref' on '{}'", stringify!($type), v.to_type()),
                    };
                }
            }

            impl AsMut<$type> for crate::Number {
                fn as_mut(&mut self) -> &mut $type {
                    return match self {
                        Self::Float(v) => match v {
                            Float::$name(v) => v,
                            v => panic!("called 'AsMut<{}>::as_mut' on '{}'", stringify!($type), v.to_type()),
                        },
                        v => panic!("called 'AsMut<{}>::as_mut' on '{}'", stringify!($type), v.to_type()),
                    };
                }
            }
        )*

        ///
        /// Float: Value
        ///
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Float {
            $($name($type),)*
        }

        impl Float {
            pub fn to_type(&self) -> crate::Type {
                return match self {
                    $(Self::$name(_) => crate::Type::Number(crate::NumberType::Float(crate::FloatType::$name(crate::$type_name))),)*
                };
            }

            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::$name(_) => true,
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $type {
                    return match self {
                        Self::$name(v) => *v,
                        _ => panic!("called '{}' on '{}'", stringify!($to_type), stringify!($type)),
                    };
                }

                pub fn $set_value(&mut self, value: $type) {
                    *self = Self::$name(value);
                }
            )*
        }

        impl crate::ToValue for crate::Float {
            fn to_value(self) -> crate::Value {
                return crate::Value::Number(crate::Number::Float(self.clone()));
            }
        }

        impl crate::AsValue for crate::Float {
            fn as_value(&self) -> crate::Value {
                return crate::Value::Number(crate::Number::Float(*self));
            }
        }

        $(
            impl From<$type> for crate::Float {
                fn from(value: $type) -> Self {
                    return Self::$name(value);
                }
            }

            impl Into<$type> for crate::Float {
                fn into(self) -> $type {
                    return self.$to_type();
                }
            }

           impl AsRef<$type> for crate::Float {
                fn as_ref(&self) -> &$type {
                    return match self {
                        Self::$name(v) => v,
                        v => panic!("called 'AsRef<{}>::as_ref' on '{}'", stringify!($type), v.to_type()),
                    };
                }
            }

            impl AsMut<$type> for crate::Float {
                fn as_mut(&mut self) -> &mut $type {
                    return match self {
                        Self::$name(v) => v,
                        v => panic!("called 'AsMut<{}>::as_mut' on '{}'", stringify!($type), v.to_type()),
                    };
                }
            }
        )*

        impl PartialEq<crate::Value> for crate::Float {
            fn eq(&self, other: &crate::Value) -> bool {
                return other.is_float() && other.as_number().as_float() == self;
            }
        }

        impl std::fmt::Display for crate::Float {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    $(Self::$name(v) => write!(f, "{}", v),)*
                };
            }
        }
    };
}

float! {
    F32 F32Type is_f32 to_f32 set_f32 f32
    F64 F64Type is_f64 to_f64 set_f64 f64
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn f32() {
        let value = value_of!(300.26_f32);

        assert!(value.is_float());
        assert!(value.is_f32());
        assert_eq!(value.to_f32(), 300.26);
    }

    #[test]
    pub fn f64() {
        let value = value_of!(350.26_f64);

        assert!(value.is_float());
        assert!(value.is_f64());
        assert_eq!(value.to_f64(), 350.26);
    }
}
