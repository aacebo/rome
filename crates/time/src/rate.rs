#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Rate {
    Hz(u64),
    Period(std::time::Duration),
}

impl Rate {
    pub fn duration(&self) -> std::time::Duration {
        match self {
            Self::Period(v) => *v,
            Self::Hz(v) => std::time::Duration::from_nanos(1_000_000_000 / v),
        }
    }
}

impl From<u64> for Rate {
    fn from(value: u64) -> Self {
        assert!(value > 0);
        Self::Hz(value)
    }
}

impl From<std::time::Duration> for Rate {
    fn from(value: std::time::Duration) -> Self {
        Self::Period(value)
    }
}

impl std::fmt::Display for Rate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hz(v) => write!(f, "{}Hz", v),
            Self::Period(v) => write!(f, "{:?}", v),
        }
    }
}
