use crate::domain::job::Job;
use crate::domain::state::JobState;

/// Scheduler input snapshot.
/// Represents the full scheduling view at a single decision point.
#[derive(Debug)]
pub struct SchedulerInput<'a> {
    pub queued_jobs: &'a [Job],
    pub running_count: usize,
    pub max_concurrency: usize,
}

/// Scheduler decision output.
#[derive(Debug, PartialEq)]
pub struct SchedulerDecision {
    pub selected_job_ids: Vec<uuid::Uuid>,
    pub remaining_capacity: usize,
}

/// Deterministic scheduler selection.
///
/// Rules:
/// 1. Never exceed max_concurrency.
/// 2. Only jobs in Queued state are eligible.
/// 3. Order by:
///    - priority DESC
///    - created_at ASC
/// 4. FIFO within same priority.
/// 5. If capacity is zero, select nothing.
pub fn select_jobs(input: SchedulerInput) -> SchedulerDecision {
    let available_capacity = input
        .max_concurrency
        .saturating_sub(input.running_count);

    if available_capacity == 0 {
        return SchedulerDecision {
            selected_job_ids: Vec::new(),
            remaining_capacity: 0,
        };
    }

    let mut candidates: Vec<&Job> = input
        .queued_jobs
        .iter()
        .filter(|job| job.state == JobState::Queued)
        .collect();

    candidates.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| a.created_at.cmp(&b.created_at))
    });

    let selected: Vec<uuid::Uuid> = candidates
        .into_iter()
        .take(available_capacity)
        .map(|job| job.id)
        .collect();

    SchedulerDecision {
        remaining_capacity: available_capacity - selected.len(),
        selected_job_ids: selected,
    }
}
