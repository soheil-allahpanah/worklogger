use axum::{
    extract::{Path, State},
    Extension, Json,
};
use domain::actor::ActorContext;
use use_cases::GetWorklogCommand;
use uuid::Uuid;

use crate::dto::WorklogJson;
use crate::error::ApiResult;
use crate::mapper::worklog_to_json;
use crate::state::AppState;

pub async fn get(
    State(state): State<AppState>,
    Extension(actor): Extension<ActorContext>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorklogJson>> {
    let command = GetWorklogCommand {
        user_id: actor.user_id(),
        id,
    };
    let worklog = state.get_worklog().execute(command).await?;
    Ok(Json(worklog_to_json(worklog)))
}
