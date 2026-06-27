use axum::{extract::State, http::StatusCode, Json};

use crate::dto::LogoutRequest;
use crate::error::ApiResult;
use crate::state::AppState;
use use_cases::RevokeRefreshTokenCommand;

pub async fn logout(
    State(state): State<AppState>,
    Json(body): Json<LogoutRequest>,
) -> ApiResult<StatusCode> {
    state
        .revoke_refresh_token()
        .execute(RevokeRefreshTokenCommand {
            refresh_token: body.refresh_token,
        })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
