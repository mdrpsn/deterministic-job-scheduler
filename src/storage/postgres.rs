use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::domain::failure::{Failure, FailureKind};
use crate::domain::job::Job;
use crate::domain::state::JobState;
use crate::storage::repository::{JobRepository, RepositoryError};

pub struct PostgresJobRepository {
    pool: PgPool,
}

impl PostgresJobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl JobRepository for PostgresJobRepository {
    async fn fetch_queued_jobs(&self) -> Result<Vec<Job>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                id,
                payload,
                priority,
                state,
                attempt,
                max_attempts,
                failure_type,
                failure_reason,
                created_at,
                updated_at
            FROM jobs
            WHERE state = 'queued'
            ORDER BY priority DESC, created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(row_to_job).collect()
    }

    async fn fetch_running_jobs(&self) -> Result<Vec<Job>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                id,
                payload,
                priority,
                state,
                attempt,
                max_attempts,
                failure_type,
                failure_reason,
                created_at,
                updated_at
            FROM jobs
            WHERE state = 'running'
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(row_to_job).collect()
    }

    async fn insert_job(&self, job: &Job) -> Result<(), RepositoryError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO jobs (
                id,
                payload,
                priority,
                state,
                attempt,
                max_attempts,
                created_at,
                updated_at
            )
            VALUES (, , , , , , , )
            "#,
            job.id,
            job.payload,
            job.priority,
            state_to_str(job.state),
            job.attempt as i32,
            job.max_attempts as i32,
            job.created_at,
            job.updated_at
        )
        .execute(&mut *tx)
        .await?;

        insert_event(&mut tx, job.id, job.state, job.state, "job created").await?;
        tx.commit().await?;
        Ok(())
    }

    async fn update_job_state(
        &self,
        job_id: Uuid,
        from: JobState,
        to: JobState,
        failure: Option<&Failure>,
    ) -> Result<(), RepositoryError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"
            UPDATE jobs
            SET
                state = ,
                attempt = attempt + CASE WHEN  THEN 1 ELSE 0 END,
                failure_type = ,
                failure_reason = ,
                updated_at = now()
            WHERE id =  AND state = 
            "#,
            state_to_str(to),
            to == JobState::Failed,
            failure.map(|f| failure_kind_to_str(f.kind)),
            failure.map(|f| f.reason.as_str()),
            job_id,
            state_to_str(from)
        )
        .execute(&mut *tx)
        .await?;

        insert_event(&mut tx, job_id, from, to, "state transition").await?;
        tx.commit().await?;
        Ok(())
    }
}

fn row_to_job(row: sqlx::postgres::PgRow) -> Result<Job, RepositoryError> {
    use sqlx::Row;

    let failure = match row.try_get::<Option<String>, _>("failure_type")? {
        Some(kind) => {
            let reason: String = row.try_get("failure_reason")?;
            Some(Failure {
                kind: str_to_failure_kind(&kind),
                reason,
            })
        }
        None => None,
    };

    Ok(Job {
        id: row.try_get("id")?,
        payload: row.try_get("payload")?,
        priority: row.try_get("priority")?,
        state: str_to_state(row.try_get("state")?),
        attempt: row.try_get::<i32, _>("attempt")? as u32,
        max_attempts: row.try_get::<i32, _>("max_attempts")? as u32,
        failure,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn state_to_str(state: JobState) -> &'static str {
    match state {
        JobState::Queued => "queued",
        JobState::Running => "running",
        JobState::Succeeded => "succeeded",
        JobState::Failed => "failed",
        JobState::Cancelled => "cancelled",
    }
}

fn str_to_state(value: String) -> JobState {
    match value.as_str() {
        "queued" => JobState::Queued,
        "running" => JobState::Running,
        "succeeded" => JobState::Succeeded,
        "failed" => JobState::Failed,
        "cancelled" => JobState::Cancelled,
        _ => JobState::Failed,
    }
}

fn failure_kind_to_str(kind: FailureKind) -> &'static str {
    match kind {
        FailureKind::UserError => "user_error",
        FailureKind::SystemError => "system_error",
        FailureKind::Timeout => "timeout",
    }
}

fn str_to_failure_kind(value: &str) -> FailureKind {
    match value {
        "user_error" => FailureKind::UserError,
        "system_error" => FailureKind::SystemError,
        "timeout" => FailureKind::Timeout,
        _ => FailureKind::SystemError,
    }
}

async fn insert_event(
    tx: &mut Transaction<'_, Postgres>,
    job_id: Uuid,
    from: JobState,
    to: JobState,
    reason: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO job_events (job_id, from_state, to_state, event_reason)
        VALUES (, , , )
        "#,
        job_id,
        state_to_str(from),
        state_to_str(to),
        reason
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
