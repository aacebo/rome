use crate::ToType;

#[macro_export]
macro_rules! map {
    // empty -> HashMap
    () => {{
        ::std::collections::HashMap::new()
    }};

    // default HashMap with tuple-ish `{ {k, v}, ... }` syntax
    ( $( { $key:expr, $value:expr } ),+ $(,)? ) => {{
        ::std::collections::HashMap::from([
            $(($key, $value),)+
        ])
    }};

    // default HashMap with `k => v` syntax
    ( $( $key:expr => $value:expr ),+ $(,)? ) => {{
        ::std::collections::HashMap::from([
            $(($key, $value),)+
        ])
    }};
}

#[macro_export]
macro_rules! btree_map {
    // empty -> BTreeMap
    () => {{
        ::std::collections::BTreeMap::new()
    }};

    // default BTreeMap with tuple-ish `{ {k, v}, ... }` syntax
    ( $( { $key:expr, $value:expr } ),+ $(,)? ) => {{
        ::std::collections::BTreeMap::from([
            $(($key, $value),)+
        ])
    }};

    // default BTreeMap with `k => v` syntax
    ( $( $key:expr => $value:expr ),+ $(,)? ) => {{
        ::std::collections::BTreeMap::from([
            $(($key, $value),)+
        ])
    }};
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Map {
    pub(crate) ty: crate::MapType,
    pub(crate) data: std::collections::BTreeMap<crate::Value, crate::Value>,
}

impl Map {
    pub fn new(ty: &crate::MapType) -> Self {
        return Self {
            ty: ty.clone(),
            data: std::collections::BTreeMap::new(),
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Map(self.ty.clone());
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, crate::Value, crate::Value> {
        return self.data.iter();
    }

    pub fn keys(&self) -> Vec<crate::Value> {
        return self.data.clone().into_keys().collect::<Vec<_>>();
    }

    pub fn values(&self) -> Vec<crate::Value> {
        return self.data.clone().into_values().collect::<Vec<_>>();
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn data(&self) -> &std::collections::BTreeMap<crate::Value, crate::Value> {
        return &self.data;
    }

    pub fn has(&self, key: &crate::Value) -> bool {
        return self.data.contains_key(key);
    }

    pub fn get(&self, key: &crate::Value) -> Option<&crate::Value> {
        return self.data.get(key);
    }

    pub fn get_mut(&mut self, key: &crate::Value) -> Option<&mut crate::Value> {
        return self.data.get_mut(key);
    }

    pub fn set(&mut self, value: crate::Value) {
        self.data = value.to_map().data().clone();
    }

    pub fn set_key_value(&mut self, key: crate::Value, value: crate::Value) {
        self.data.insert(key, value);
    }
}

impl crate::ToType for Map {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Map(self.ty.clone());
    }
}

impl crate::ToValue for Map {
    fn to_value(self) -> crate::Value {
        return crate::Value::Map(self.clone());
    }
}

impl crate::AsValue for Map {
    fn as_value(&self) -> crate::Value {
        return crate::Value::Map(self.clone());
    }
}

impl std::ops::Index<&crate::Value> for Map {
    type Output = crate::Value;

    fn index(&self, index: &crate::Value) -> &Self::Output {
        return self.data.index(index);
    }
}

impl std::ops::IndexMut<&crate::Value> for Map {
    fn index_mut(&mut self, index: &crate::Value) -> &mut Self::Output {
        return self.data.get_mut(index).unwrap();
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for (key, value) in &self.data {
            write!(f, "\n\t{}: {}", key, value)?;
        }

        if self.data.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}

impl<K, V> crate::ToValue for std::collections::HashMap<K, V>
where
    K: crate::TypeOf + crate::ToValue,
    V: crate::TypeOf + crate::ToValue,
{
    fn to_value(self) -> crate::Value {
        let ty = self.to_type();
        let mut value = Map::new(ty.as_map());

        for (k, v) in self {
            value.set_key_value(k.to_value(), v.to_value());
        }

        return value.to_value();
    }
}

impl<K, V> crate::AsValue for std::collections::HashMap<K, V>
where
    K: crate::TypeOf + crate::AsValue,
    V: crate::TypeOf + crate::AsValue,
{
    fn as_value(&self) -> crate::Value {
        let ty = self.to_type();
        let mut value = Map::new(ty.as_map());

        for (k, v) in self {
            value.set_key_value(k.as_value(), v.as_value());
        }

        return value.as_value();
    }
}

impl<K, V> crate::ToValue for std::collections::BTreeMap<K, V>
where
    K: crate::TypeOf + crate::ToValue,
    V: crate::TypeOf + crate::ToValue,
{
    fn to_value(self) -> crate::Value {
        let ty = self.to_type();
        let mut value = Map::new(ty.as_map());

        for (k, v) in self {
            value.set_key_value(k.to_value(), v.to_value());
        }

        return value.to_value();
    }
}

impl<K, V> crate::AsValue for std::collections::BTreeMap<K, V>
where
    K: crate::TypeOf + crate::AsValue,
    V: crate::TypeOf + crate::AsValue,
{
    fn as_value(&self) -> crate::Value {
        let ty = self.to_type();
        let mut value = Map::new(ty.as_map());

        for (k, v) in self {
            value.set_key_value(k.as_value(), v.as_value());
        }

        return value.as_value();
    }
}

#[cfg(test)]
mod test {
    use crate::value_of;

    #[test]
    pub fn to_value() {
        let value = value_of!(btree_map! {
            "hello".to_string() => 123,
            "world".to_string() => 111
        });

        assert!(value.is_map());
        assert_eq!(value.len(), 2);
        assert_eq!(value["hello"], value_of!(123));
    }
}
