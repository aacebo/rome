#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(transparent)]
pub struct Tick(u64);

impl Tick {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}
