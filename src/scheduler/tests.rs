use chrono::{TimeZone, Utc};
use uuid::Uuid;

use crate::domain::job::Job;
use crate::domain::state::JobState;
use crate::scheduler::{select_jobs, SchedulerInput, SchedulerDecision};

fn job(id: u8, priority: i32, created_at: i64) -> Job {
    Job {
        id: Uuid::from_u128(id as u128),
        payload: serde_json::json!({}),
        priority,
        state: JobState::Queued,
        attempt: 0,
        max_attempts: 3,
        failure: None,
        created_at: Utc.timestamp_opt(created_at, 0).unwrap(),
        updated_at: Utc.timestamp_opt(created_at, 0).unwrap(),
    }
}

#[test]
fn respects_concurrency_limit() {
    let jobs = vec![job(1, 0, 1), job(2, 0, 2), job(3, 0, 3)];

    let decision = select_jobs(SchedulerInput {
        queued_jobs: &jobs,
        running_count: 1,
        max_concurrency: 2,
    });

    assert_eq!(decision.selected_job_ids.len(), 1);
    assert_eq!(decision.remaining_capacity, 0);
}

#[test]
fn orders_by_priority_then_fifo() {
    let jobs = vec![
        job(1, 1, 10),
        job(2, 2, 20),
        job(3, 2, 5),
        job(4, 1, 1),
    ];

    let decision = select_jobs(SchedulerInput {
        queued_jobs: &jobs,
        running_count: 0,
        max_concurrency: 10,
    });

    let expected = vec![
        Uuid::from_u128(3),
        Uuid::from_u128(2),
        Uuid::from_u128(4),
        Uuid::from_u128(1),
    ];

    assert_eq!(decision.selected_job_ids, expected);
}

#[test]
fn selects_nothing_when_saturated() {
    let jobs = vec![job(1, 0, 1)];

    let decision = select_jobs(SchedulerInput {
        queued_jobs: &jobs,
        running_count: 5,
        max_concurrency: 5,
    });

    assert_eq!(
        decision,
        SchedulerDecision {
            selected_job_ids: vec![],
            remaining_capacity: 0
        }
    );
}
