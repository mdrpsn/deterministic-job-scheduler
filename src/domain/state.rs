use crate::domain::failure::Failure;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum JobState {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug)]
pub enum StateTransitionError {
    InvalidTransition {
        from: JobState,
        to: JobState,
    },
}

impl JobState {
    pub fn transition(
        self,
        next: JobState,
        failure: Option<&Failure>,
    ) -> Result<JobState, StateTransitionError> {
        use JobState::*;

        let valid = match (self, next) {
            (Queued, Running) => true,
            (Running, Succeeded) => true,
            (Running, Failed) => true,
            (Queued, Cancelled) => true,
            (Running, Cancelled) => true,
            _ => false,
        };

        if !valid {
            return Err(StateTransitionError::InvalidTransition {
                from: self,
                to: next,
            });
        }

        if next == Failed && failure.is_none() {
            return Err(StateTransitionError::InvalidTransition {
                from: self,
                to: next,
            });
        }

        Ok(next)
    }
}
