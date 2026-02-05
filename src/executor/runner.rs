use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio::time::timeout;
use uuid::Uuid;

use crate::domain::failure::Failure;
use crate::domain::state::JobState;
use crate::storage::repository::JobRepository;

/// Trait representing executable job logic.
/// v1 uses a simulated handler; real work can replace this later.
#[async_trait::async_trait]
pub trait JobHandler: Send + Sync + 'static {
    async fn execute(&self, job_id: Uuid) -> Result<(), Failure>;
}

pub struct Executor<R, H>
where
    R: JobRepository,
    H: JobHandler,
{
    repository: Arc<R>,
    handler: Arc<H>,
    job_timeout: Duration,
}

impl<R, H> Executor<R, H>
where
    R: JobRepository,
    H: JobHandler,
{
    pub fn new(
        repository: Arc<R>,
        handler: Arc<H>,
        job_timeout: Duration,
    ) -> Self {
        Self {
            repository,
            handler,
            job_timeout,
        }
    }

    /// Spawn execution of a single job.
    /// Caller is responsible for respecting concurrency limits.
    pub fn spawn(&self, job_id: Uuid) -> JoinHandle<()> {
        let repo = Arc::clone(&self.repository);
        let handler = Arc::clone(&self.handler);
        let timeout_duration = self.job_timeout;

        tokio::spawn(async move {
            let result = timeout(timeout_duration, handler.execute(job_id)).await;

            match result {
                Ok(Ok(())) => {
                    let _ = repo
                        .update_job_state(
                            job_id,
                            JobState::Running,
                            JobState::Succeeded,
                            None,
                        )
                        .await;
                }

                Ok(Err(failure)) => {
                    let _ = repo
                        .update_job_state(
                            job_id,
                            JobState::Running,
                            JobState::Failed,
                            Some(&failure),
                        )
                        .await;
                }

                Err(_) => {
                    let failure =
                        Failure::timeout("job execution exceeded timeout");

                    let _ = repo
                        .update_job_state(
                            job_id,
                            JobState::Running,
                            JobState::Failed,
                            Some(&failure),
                        )
                        .await;
                }
            }
        })
    }
}
