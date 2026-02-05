use std::sync::Arc;
use std::time::Duration;

use tokio::time::sleep;
use tracing::{info, warn};

use crate::domain::state::JobState;
use crate::scheduler::{select_jobs, SchedulerInput};
use crate::storage::repository::JobRepository;
use crate::executor::runner::{Executor, JobHandler};

pub struct Orchestrator<R, H>
where
    R: JobRepository,
    H: JobHandler,
{
    repository: Arc<R>,
    executor: Arc<Executor<R, H>>,
    max_concurrency: usize,
    tick_interval: Duration,
}

impl<R, H> Orchestrator<R, H>
where
    R: JobRepository,
    H: JobHandler,
{
    pub fn new(
        repository: Arc<R>,
        executor: Arc<Executor<R, H>>,
        max_concurrency: usize,
        tick_interval: Duration,
    ) -> Self {
        Self {
            repository,
            executor,
            max_concurrency,
            tick_interval,
        }
    }

    /// Main orchestration loop.
    /// This function is intended to run indefinitely.
    pub async fn run(&self) {
        loop {
            if let Err(err) = self.tick().await {
                warn!(error = ?err, "orchestration tick failed");
            }

            sleep(self.tick_interval).await;
        }
    }

    async fn tick(&self) -> Result<(), crate::orchestrator::error::OrchestrationError> {
        // 1. Snapshot current system state
        let queued_jobs = self.repository.fetch_queued_jobs().await?;
        let running_jobs = self.repository.fetch_running_jobs().await?;

        let running_count = running_jobs.len();

        // 2. Scheduler decision (pure)
        let decision = select_jobs(SchedulerInput {
            queued_jobs: &queued_jobs,
            running_count,
            max_concurrency: self.max_concurrency,
        });

        if decision.selected_job_ids.is_empty() {
            return Ok(());
        }

        info!(
            selected = decision.selected_job_ids.len(),
            running = running_count,
            "scheduler selected jobs"
        );

        // 3. Transition jobs to Running and spawn execution
        for job_id in decision.selected_job_ids {
            let transitioned = self
                .repository
                .update_job_state(
                    job_id,
                    JobState::Queued,
                    JobState::Running,
                    None,
                )
                .await;

            if transitioned.is_err() {
                warn!(
                    job_id = %job_id,
                    "failed to transition job to running (likely raced)"
                );
                continue;
            }

            // 4. Hand off to executor
            self.executor.spawn(job_id);
        }

        Ok(())
    }
}
