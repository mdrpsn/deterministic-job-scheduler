use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::job::Job;
use crate::domain::failure::Failure;
use crate::domain::state::JobState;

#[async_trait]
pub trait JobRepository {
    async fn fetch_queued_jobs(&self) -> Result<Vec<Job>, RepositoryError>;
    async fn fetch_running_jobs(&self) -> Result<Vec<Job>, RepositoryError>;

    async fn insert_job(&self, job: &Job) -> Result<(), RepositoryError>;

    async fn update_job_state(
        &self,
        job_id: Uuid,
        from: JobState,
        to: JobState,
        failure: Option<&Failure>,
    ) -> Result<(), RepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}
