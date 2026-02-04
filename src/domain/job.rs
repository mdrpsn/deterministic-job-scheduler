use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::state::JobState;
use crate::domain::failure::Failure;

#[derive(Debug, Clone)]
pub struct Job {
    pub id: Uuid,
    pub payload: serde_json::Value,
    pub priority: i32,

    pub state: JobState,

    pub attempt: u32,
    pub max_attempts: u32,

    pub failure: Option<Failure>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Job {
    pub fn can_retry(&self) -> bool {
        self.attempt < self.max_attempts
    }
}
