use axum::{
    extract::State,
    Extension, Json,
};
use domain::actor::ActorContext;

use crate::dto::{FilterWorklogsRequest, TagStatsJson};
use crate::error::ApiResult;
use crate::mapper::{filter_worklogs_request_to_command, tag_stats_to_json};
use crate::routes::controllers::validate::validate_filter;
use crate::state::AppState;

pub async fn tag_stats(
    State(state): State<AppState>,
    Extension(actor): Extension<ActorContext>,
    Json(mut request): Json<FilterWorklogsRequest>,
) -> ApiResult<Json<TagStatsJson>> {
    validate_filter(&mut request)?;
    let command = filter_worklogs_request_to_command(request, actor.user_id());
    let response = state.tag_stats().execute(command).await?;
    Ok(Json(tag_stats_to_json(response)))
}
