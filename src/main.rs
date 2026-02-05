use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::executor::Executor;
use crate::executor::sleep_handler::SleepJobHandler;
use crate::orchestrator::Orchestrator;
use crate::storage::PostgresJobRepository;

mod api;
mod config;
mod domain;
mod scheduler;
mod executor;
mod storage;
mod recovery;
mod orchestrator;
mod observability;
mod errors;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let repository = Arc::new(PostgresJobRepository::new(pool));

    let handler = Arc::new(SleepJobHandler);
    let executor = Arc::new(Executor::new(
        Arc::clone(&repository),
        handler,
        config.job_timeout,
    ));

    let orchestrator = Orchestrator::new(
        Arc::clone(&repository),
        executor,
        config.max_concurrency,
        config.scheduler_tick_interval,
    );

    orchestrator.run().await;

    Ok(())
}
