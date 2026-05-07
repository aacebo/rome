#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Str(pub(crate) std::string::String);

impl Str {
    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Str(crate::StrType);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

impl AsMut<str> for Str {
    fn as_mut(&mut self) -> &mut str {
        return &mut self.0;
    }
}

impl std::ops::Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl std::ops::DerefMut for Str {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl std::fmt::Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.0);
    }
}

impl From<std::string::String> for crate::Value {
    fn from(value: std::string::String) -> Self {
        return Self::Str(Str(value));
    }
}

impl Into<std::string::String> for crate::Value {
    fn into(self) -> std::string::String {
        return self.to_string();
    }
}

impl crate::ToValue for std::string::String {
    fn to_value(self) -> crate::Value {
        return crate::Value::Str(Str(self.clone()));
    }
}

impl crate::AsValue for std::string::String {
    fn as_value(&self) -> crate::Value {
        return crate::Value::Str(Str(self.clone()));
    }
}

impl crate::ToValue for &str {
    fn to_value(self) -> crate::Value {
        return crate::Value::Str(Str(self.to_string()));
    }
}

impl crate::AsValue for &str {
    fn as_value(&self) -> crate::Value {
        return crate::Value::Str(Str(self.to_string()));
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn str() {
        let value = value_of!("test");

        assert!(value.is_str());
        assert_eq!(value.len(), 4);
        assert_eq!(value.to_string(), "test");
    }

    #[test]
    pub fn string() {
        let value = value_of!("test".to_string());

        assert!(value.is_str());
        assert_eq!(value.len(), 4);
        assert_eq!(value.to_string(), "test");
    }
}
