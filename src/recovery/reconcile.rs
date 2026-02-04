use uuid::Uuid;

use crate::domain::job::Job;
use crate::domain::state::JobState;

/// Result of startup reconciliation.
#[derive(Debug)]
pub struct RecoveryOutcome {
    pub reconciled_job_ids: Vec<Uuid>,
    pub skipped_job_ids: Vec<Uuid>,
}

/// Pure recovery reconciliation logic.
///
/// Rules:
/// 1. Only jobs in Running are reconciled.
/// 2. Running jobs must be force-failed by the orchestrator/storage layer.
/// 3. Other states are untouched.
/// 4. This operation is idempotent.
pub fn reconcile_jobs(jobs: &[Job]) -> RecoveryOutcome {
    let mut reconciled = Vec::new();
    let mut skipped = Vec::new();

    for job in jobs {
        match job.state {
            JobState::Running => reconciled.push(job.id),
            _ => skipped.push(job.id),
        }
    }

    RecoveryOutcome {
        reconciled_job_ids: reconciled,
        skipped_job_ids: skipped,
    }
}
