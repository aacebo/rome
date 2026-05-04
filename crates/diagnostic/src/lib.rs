pub mod builder;
pub mod prelude;
pub mod severity;

pub use severity::Severity;

pub fn new() -> builder::DiagnosticBuilder {
    builder::DiagnosticBuilder::new()
}

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
pub struct DiagnosticId(u64);

impl DiagnosticId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u64> for DiagnosticId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: Option<String>,
    pub children: Vec<Self>,
    pub timestamp: std::time::SystemTime,
}
