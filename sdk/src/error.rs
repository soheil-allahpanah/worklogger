use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("network error: {0}")]
    Network(String),
    #[error("invalid API response: {0}")]
    InvalidResponse(String),
    #[error("server error: {0}")]
    Server(String),
}

pub type SdkResult<T> = Result<T, SdkError>;
