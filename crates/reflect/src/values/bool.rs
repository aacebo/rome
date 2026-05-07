impl From<bool> for crate::Value {
    fn from(value: bool) -> Self {
        return Self::Bool(value);
    }
}

impl crate::ToValue for bool {
    fn to_value(self) -> crate::Value {
        return crate::Value::Bool(self);
    }
}

impl crate::AsValue for bool {
    fn as_value(&self) -> crate::Value {
        return crate::Value::Bool(*self);
    }
}

impl crate::Value {
    pub fn is_true(&self) -> bool {
        return self.is_bool() && self.to_bool() == true;
    }

    pub fn is_false(&self) -> bool {
        return self.is_bool() && self.to_bool() == false;
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn truthy() {
        let value = value_of!(true);

        assert!(value.is_bool());
        assert!(value.is_true());
        assert!(value.to_bool());
    }

    #[test]
    pub fn falsy() {
        let value = value_of!(false);

        assert!(value.is_bool());
        assert!(value.is_false());
        assert!(!value.to_bool());
    }
}
