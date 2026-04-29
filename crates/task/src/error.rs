/// Errors that can occur during task execution or when awaiting a task
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError {
    /// Task was cancelled before completion
    Cancelled,

    /// Task panicked during execution
    Panic(String),

    /// Custom error with a message
    Custom(String),

    /// Task handle was dropped without sending a result
    Dropped,
}

impl TaskError {
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled)
    }

    pub fn is_panic(&self) -> bool {
        matches!(self, Self::Panic(_))
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    pub fn is_dropped(&self) -> bool {
        matches!(self, Self::Dropped)
    }

    /// Create a custom error from any error type
    pub fn custom<E: std::error::Error>(err: E) -> Self {
        Self::Custom(err.to_string())
    }

    /// Create a panic error from panic payload
    pub fn panic<S: Into<String>>(msg: S) -> Self {
        Self::Panic(msg.into())
    }
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => write!(f, "task cancelled"),
            Self::Panic(msg) => write!(f, "task panicked: {}", msg),
            Self::Custom(msg) => write!(f, "{}", msg),
            Self::Dropped => write!(f, "task handle dropped"),
        }
    }
}

impl std::error::Error for TaskError {}
