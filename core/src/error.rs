use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("tag must not be empty")]
    EmptyTag,
    #[error("tag must be at most {max} characters, got {len}")]
    TagTooLong { max: usize, len: usize },
    #[error("at least one tag is required")]
    TagsRequired,
    #[error("at most {max} tags are allowed, got {count}")]
    TooManyTags { max: usize, count: usize },
    #[error("description must not be empty")]
    EmptyDescription,
    #[error("description must be at most {max} characters, got {len}")]
    DescriptionTooLong { max: usize, len: usize },
    #[error("duration must be greater than zero")]
    InvalidDuration,
    #[error("duration must be less than 24 hours")]
    DurationTooLong,
    #[error("worklog is already deleted")]
    AlreadyDeleted,
    #[error("worklog is not deleted")]
    NotDeleted,
}


pub type DomainResult<T> = Result<T, DomainError>;
