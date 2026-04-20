pub fn new() -> DiagnosticBuilder {
    DiagnosticBuilder::new()
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
    pub severity: Severity,
    pub message: Option<String>,
    pub children: Vec<Self>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Default)]
pub struct DiagnosticBuilder {
    severity: Severity,
    message: Option<String>,
    children: Vec<Diagnostic>,
}

impl DiagnosticBuilder {
    pub fn new() -> Self {
        Self::default()
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
            severity: self.severity,
            message: self.message,
            children: self.children,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug)]
pub struct DiagnosticBuffer(Vec<Diagnostic>);

impl DiagnosticBuffer {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn first(&self) -> Option<&Diagnostic> {
        self.0.first()
    }

    pub fn last(&self) -> Option<&Diagnostic> {
        self.0.last()
    }

    pub fn read(&mut self) -> Option<Diagnostic> {
        self.0.pop()
    }

    pub fn write(&mut self, diagnostic: Diagnostic) -> &mut Self {
        self.0.push(diagnostic);
        self
    }
}
