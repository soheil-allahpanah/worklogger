use axum::{extract::State, http::StatusCode, Json};

use crate::dto::{AuthTokensJson, RefreshTokenRequest};
use crate::error::ApiResult;
use crate::state::AppState;
use use_cases::RefreshAccessTokenCommand;

pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshTokenRequest>,
) -> ApiResult<(StatusCode, Json<AuthTokensJson>)> {
    let response = state
        .refresh_access_token()
        .execute(RefreshAccessTokenCommand {
            refresh_token: body.refresh_token,
        })
        .await?;
    Ok((StatusCode::OK, Json(AuthTokensJson::from_response(response))))
}
