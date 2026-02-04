use std::time::Duration;

use tokio::time::sleep;
use uuid::Uuid;

use crate::domain::failure::Failure;
use crate::executor::runner::JobHandler;

/// Simple v1 job handler that simulates work with a sleep.
pub struct SleepJobHandler;

#[async_trait::async_trait]
impl JobHandler for SleepJobHandler {
    async fn execute(&self, _job_id: Uuid) -> Result<(), Failure> {
        sleep(Duration::from_secs(1)).await;
        Ok(())
    }
}
