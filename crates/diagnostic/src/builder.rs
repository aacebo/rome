use super::*;

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
            timestamp: std::time::SystemTime::now(),
        }
    }
}
