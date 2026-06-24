use domain::error::DomainError;
use domain::traits::RepositoryError;
use thiserror::Error;
use uuid::Error as UuidError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuthError {
    #[error("invalid or expired token")]
    InvalidToken,
    #[error("user account is disabled or deleted")]
    UserInactive,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UseCaseError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error("export failed: {0}")]
    Export(String),
}


#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("invalid worklog ID: {0}")]
    InvalidWorklogIdFormat(#[from] UuidError),
    #[error("worklog ID is required")]
    WorklogIdRequired,
    #[error("invalid Jalali date: {0}")]
    InvalidJalaliDate(String),
    #[error("duration must be greater than zero")]
    DurationRequired,
    #[error("duration must be less than 24 hours")]
    DurationTooLong,
    #[error("at least one tag is required")]
    TagsRequired,
    #[error("at most {max} tags are allowed, got {count}")]
    TooManyTags { max: usize, count: usize },
    #[error("tag must not be empty")]
    EmptyTag,
    #[error("description is required")]
    DescriptionRequired,
}

pub type UseCaseResult<T> = Result<T, UseCaseError>;
