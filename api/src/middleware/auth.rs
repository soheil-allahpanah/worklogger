use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::error::ApiError;
use crate::state::AppState;

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = extract_bearer_token(request.headers().get(AUTHORIZATION))?;
    let actor = state.authenticate_token().execute(&token).await?;
    request.extensions_mut().insert(actor);
    Ok(next.run(request).await)
}

fn extract_bearer_token(header: Option<&axum::http::HeaderValue>) -> Result<String, ApiError> {
    let value = header
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiError::unauthorized("missing Authorization header"))?;

    let token = value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))
        .ok_or_else(|| ApiError::unauthorized("Authorization header must use Bearer scheme"))?;

    let token = token.trim();
    if token.is_empty() {
        return Err(ApiError::unauthorized("missing bearer token"));
    }

    Ok(token.to_owned())
}
