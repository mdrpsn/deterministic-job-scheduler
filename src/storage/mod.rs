pub mod repository;
pub mod postgres;

pub use repository::{JobRepository, RepositoryError};
pub use postgres::PostgresJobRepository;
