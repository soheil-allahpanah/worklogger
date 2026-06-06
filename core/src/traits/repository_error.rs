use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RepositoryError {
    #[error("failed to persist worklog")]
    PersistFailed,
    #[error("failed to query worklogs")]
    QueryFailed,
    #[error("worklog not found")]
    NotFound,
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
