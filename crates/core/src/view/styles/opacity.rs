/// 0.0 = fully transparent, 1.0 = fully opaque.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Opacity(f32);

impl Opacity {
    pub fn new(opacity: f32) -> Self {
        Self(opacity)
    }
}

impl Default for Opacity {
    fn default() -> Self {
        Self::new(1.0)
    }
}
