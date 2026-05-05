#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Visibility(bool);

impl Visibility {
    pub fn new(visible: bool) -> Self {
        Self(visible)
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Self::new(true)
    }
}
