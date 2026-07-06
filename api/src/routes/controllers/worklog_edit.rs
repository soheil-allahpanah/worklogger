use axum::{
    extract::{Path, State},
    Extension, Json,
};
use domain::actor::ActorContext;
use uuid::Uuid;

use crate::dto::{EditWorklogRequest, WorklogJson};
use crate::error::ApiResult;
use crate::mapper::{edit_worklog_request_to_command, worklog_to_json};
use crate::state::AppState;

pub async fn edit(
    State(state): State<AppState>,
    Extension(actor): Extension<ActorContext>,
    Path(id): Path<Uuid>,
    Json(body): Json<EditWorklogRequest>,
) -> ApiResult<Json<WorklogJson>> {
    let command = edit_worklog_request_to_command(body, actor.user_id(), id);
    let worklog = state.edit_worklog().execute(command).await?;
    Ok(Json(worklog_to_json(worklog)))
}
