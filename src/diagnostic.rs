pub fn new(id: DiagnosticId) -> DiagnosticBuilder {
    DiagnosticBuilder::new(id)
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
#[serde(rename_all = "snake_case")]
pub enum Severity {
    #[default]
    Info,
    Warn,
    Error,
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
    pub id: DiagnosticId,
    pub severity: Severity,
    pub message: Option<String>,
    pub children: Vec<Self>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Default)]
pub struct DiagnosticBuilder {
    id: DiagnosticId,
    severity: Severity,
    message: Option<String>,
    children: Vec<Diagnostic>,
}

impl DiagnosticBuilder {
    pub fn new(id: DiagnosticId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn child(mut self, child: Diagnostic) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self) -> Diagnostic {
        Diagnostic {
            id: self.id,
            severity: self.severity,
            message: self.message,
            children: self.children,
            timestamp: chrono::Utc::now(),
        }
    }
}
