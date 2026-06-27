use axum::{extract::State, http::StatusCode, Json};

use crate::dto::{AuthTokensJson, LoginRequest};
use crate::error::ApiResult;
use crate::state::AppState;
use use_cases::LoginCommand;

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> ApiResult<(StatusCode, Json<AuthTokensJson>)> {
    let response = state
        .login()
        .execute(LoginCommand {
            login: body.login,
            password: body.password,
        })
        .await?;
    Ok((StatusCode::OK, Json(AuthTokensJson::from_response(response))))
}
