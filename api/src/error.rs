use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use domain::traits::RepositoryError;
use serde::Serialize;
use use_cases::error::{AuthError, UseCaseError};

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Vec<String>>,
}

pub struct ApiError {
    status: StatusCode,
    message: String,
    details: Option<Vec<String>>,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
            details: None,
        }
    }

    pub fn bad_request_with_details(message: impl Into<String>, details: Vec<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
            details: Some(details),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
            details: None,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
            details: None,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: message.into(),
            details: None,
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            message: message.into(),
            details: None,
        }
    }
}

impl From<UseCaseError> for ApiError {
    fn from(err: UseCaseError) -> Self {
        match err {
            UseCaseError::Validation(e) => Self::bad_request(e.to_string()),
            UseCaseError::Domain(e) => Self::bad_request(e.to_string()),
            UseCaseError::Auth(AuthError::InvalidToken) => {
                Self::unauthorized("invalid or expired token")
            }
            UseCaseError::Auth(AuthError::UserInactive) => {
                Self::forbidden("user account is disabled or deleted")
            }
            UseCaseError::Repository(RepositoryError::NotFound) => {
                Self::not_found("worklog not found")
            }
            UseCaseError::Repository(RepositoryError::UserNotFound) => {
                Self::not_found("user not found")
            }
            UseCaseError::Repository(RepositoryError::TokenNotFound) => {
                Self::unauthorized("invalid or expired token")
            }
            UseCaseError::Repository(_) => Self::internal("database error"),
            UseCaseError::Export(msg) => Self::internal(msg),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorBody {
            error: self.message,
            details: self.details,
        };
        (self.status, Json(body)).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
