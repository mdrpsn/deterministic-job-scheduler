use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub max_concurrency: usize,
    pub scheduler_tick_interval: Duration,
    pub job_timeout: Duration,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url =
            std::env::var(\"DATABASE_URL\").expect(\"DATABASE_URL must be set\");

        let max_concurrency = std::env::var(\"MAX_CONCURRENCY\")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let scheduler_tick_interval = std::env::var(\"SCHEDULER_TICK_MS\")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(500));

        let job_timeout = std::env::var(\"JOB_TIMEOUT_SECS\")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(5));

        Self {
            database_url,
            max_concurrency,
            scheduler_tick_interval,
            job_timeout,
        }
    }
}
