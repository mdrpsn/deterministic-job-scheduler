use chrono::Utc;
use uuid::Uuid;

use crate::domain::job::Job;
use crate::domain::state::JobState;
use crate::recovery::reconcile_jobs;

fn job(id: u8, state: JobState) -> Job {
    Job {
        id: Uuid::from_u128(id as u128),
        payload: serde_json::json!({}),
        priority: 0,
        state,
        attempt: 0,
        max_attempts: 3,
        failure: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn reconciles_only_running_jobs() {
    let jobs = vec![
        job(1, JobState::Queued),
        job(2, JobState::Running),
        job(3, JobState::Succeeded),
        job(4, JobState::Running),
    ];

    let outcome = reconcile_jobs(&jobs);

    assert_eq!(
        outcome.reconciled_job_ids,
        vec![Uuid::from_u128(2), Uuid::from_u128(4)]
    );

    assert_eq!(
        outcome.skipped_job_ids,
        vec![Uuid::from_u128(1), Uuid::from_u128(3)]
    );
}
