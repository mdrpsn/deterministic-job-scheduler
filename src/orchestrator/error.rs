#[derive(Debug, thiserror::Error)]
pub enum OrchestrationError {
    #[error("repository error: {0}")]
    Repository(#[from] crate::storage::repository::RepositoryError),
}
