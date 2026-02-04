#[derive(Debug, Clone)]
pub struct Failure {
    pub kind: FailureKind,
    pub reason: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FailureKind {
    UserError,
    SystemError,
    Timeout,
}

impl Failure {
    pub fn user(reason: impl Into<String>) -> Self {
        Self {
            kind: FailureKind::UserError,
            reason: reason.into(),
        }
    }

    pub fn system(reason: impl Into<String>) -> Self {
        Self {
            kind: FailureKind::SystemError,
            reason: reason.into(),
        }
    }

    pub fn timeout(reason: impl Into<String>) -> Self {
        Self {
            kind: FailureKind::Timeout,
            reason: reason.into(),
        }
    }
}
