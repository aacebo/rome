use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct MetaData(BTreeMap<String, crate::Value<'static>>);

impl MetaData {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, crate::Value<'static>> {
        self.0.iter()
    }

    pub fn has(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&crate::Value<'static>> {
        self.0.get(key)
    }

    pub fn merge(mut self, other: &Self) -> Self {
        for (key, value) in &other.0 {
            self.0.insert(key.clone(), value.clone());
        }
        self
    }
}

impl<const N: usize, V: crate::ToValue + 'static> From<[(&str, V); N]> for MetaData {
    fn from(items: [(&str, V); N]) -> Self {
        let mut data = BTreeMap::new();

        for (key, value) in items {
            let static_val: crate::Value<'static> = value.to_value().to_static();
            data.insert(key.to_string(), static_val);
        }
        Self(data)
    }
}

impl std::ops::Index<&str> for MetaData {
    type Output = crate::Value<'static>;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl std::fmt::Display for MetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for (key, value) in &self.0 {
            write!(f, "\n\t{}: {}", key, value)?;
        }
        if !self.0.is_empty() {
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

impl crate::TypeOf for MetaData {
    fn type_of() -> crate::Type {
        crate::StructType::new()
            .path(crate::Path::from("ayr_reflect"))
            .name("MetaData")
            .visibility(crate::Visibility::Public(crate::Public::Full))
            .build()
            .to_type()
    }
}

impl crate::ToType for MetaData {
    fn to_type(&self) -> crate::Type {
        <Self as crate::TypeOf>::type_of()
    }
}

impl crate::ToValue for MetaData {
    fn to_value<'a>(&'a self) -> crate::Value<'a> {
        crate::Value::Dynamic(crate::Dynamic::from_object(self))
    }
}

impl crate::Object for MetaData {
    fn field(&self, name: &crate::FieldName) -> crate::Value<'_> {
        self.get(&name.to_string()).unwrap().clone()
    }
}
