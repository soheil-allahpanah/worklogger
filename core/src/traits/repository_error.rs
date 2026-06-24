use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RepositoryError {
    #[error("failed to persist worklog")]
    PersistFailed,
    #[error("failed to query worklogs")]
    QueryFailed,
    #[error("worklog not found")]
    NotFound,
    #[error("user not found")]
    UserNotFound,
    #[error("token not found")]
    TokenNotFound,
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
