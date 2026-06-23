use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};

use crate::dto::{CreateWorklogJson, CreateWorklogRequest};
use crate::error::ApiResult;
use crate::mapper::{create_worklog_request_to_command, worklog_id_to_json};
use crate::state::AppState;

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateWorklogRequest>,
) -> ApiResult<(StatusCode, Json<CreateWorklogJson>)> {
    let command = create_worklog_request_to_command(body);
    let response = state.create_worklog().execute(command).await?;
    Ok((StatusCode::CREATED, Json(worklog_id_to_json(response.id))))
}
