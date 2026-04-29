#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum JoinError {
    Cancelled,
}

impl std::fmt::Debug for JoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}
