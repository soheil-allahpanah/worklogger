use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use use_cases::DeleteWorklogCommand;
use uuid::Uuid;

use crate::error::ApiResult;
use crate::state::AppState;

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let command = DeleteWorklogCommand { id };
    state.delete_worklog().execute(command).await?;
    Ok(StatusCode::NO_CONTENT)
}
