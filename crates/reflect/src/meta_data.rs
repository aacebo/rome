use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct MetaData(BTreeMap<String, crate::Value>);

impl MetaData {
    pub fn new() -> Self {
        return Self(BTreeMap::new());
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, crate::Value> {
        return self.0.iter();
    }

    pub fn has(&self, key: &str) -> bool {
        return self.0.contains_key(key);
    }

    pub fn set(&mut self, key: &str, value: &crate::Value) {
        self.0.insert(key.to_string(), value.clone());
    }

    pub fn get(&self, key: &str) -> Option<&crate::Value> {
        return self.0.get(key);
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut crate::Value> {
        return self.0.get_mut(key);
    }

    pub fn merge(mut self, other: &Self) -> Self {
        for (key, value) in &other.0 {
            self.set(&key, &value);
        }

        return self;
    }
}

impl<const N: usize, V: crate::ToValue> From<[(&str, V); N]> for MetaData {
    fn from(value: [(&str, V); N]) -> Self {
        let mut data = BTreeMap::new();

        for (key, value) in value {
            data.insert(key.to_string(), value.to_value());
        }

        return Self(data);
    }
}

impl std::ops::Index<&str> for MetaData {
    type Output = crate::Value;

    fn index(&self, index: &str) -> &Self::Output {
        return self.get(index).unwrap();
    }
}

impl std::ops::IndexMut<&str> for MetaData {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        return self.get_mut(index).unwrap();
    }
}

impl std::fmt::Display for MetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for (key, value) in &self.0 {
            write!(f, "\n\t{}: {}", key, value)?;
        }

        if self.0.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}

impl crate::ToType for MetaData {
    fn to_type(&self) -> crate::Type {
        return crate::StructType::new()
            .with_path(&crate::Path::from("zinq::reflect"))
            .with_name("MetaData")
            .with_visibility(crate::Visibility::Public(crate::Public::Full))
            .build()
            .to_type();
    }
}

impl crate::ToValue for MetaData {
    fn to_value(self) -> crate::Value {
        return crate::Dynamic::from_object(self).to_value();
    }
}

impl crate::AsValue for MetaData {
    fn as_value(&self) -> crate::Value {
        return crate::Dynamic::from_object(self.clone()).as_value();
    }
}

impl crate::Dyn for MetaData {}

impl crate::Object for MetaData {
    fn field(&self, name: &crate::FieldName) -> crate::Value {
        return self.get(name.as_str()).unwrap().clone();
    }
}
