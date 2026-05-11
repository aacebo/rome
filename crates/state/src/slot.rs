use crate::Version;

pub struct Slot<T> {
    version: Version,
    inner: T,
}

impl<T> Slot<T> {
    pub fn new(inner: T) -> Self {
        Self {
            version: Version::default(),
            inner,
        }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn take(self) -> T {
        self.inner
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn set(&mut self, next: T) {
        self.version.increment();
        self.inner = next;
    }
}

impl<T> From<T> for Slot<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> AsRef<T> for Slot<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsMut<T> for Slot<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> std::ops::Deref for Slot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::fmt::Debug for Slot<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", &self.inner)
    }
}

impl<T> serde::Serialize for Slot<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T> serde::Deserialize<'de> for Slot<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Slot::new(value))
    }
}
