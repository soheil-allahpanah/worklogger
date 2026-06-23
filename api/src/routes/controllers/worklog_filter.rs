use axum::{
    extract::{Query, State},
    Json,
};

use crate::dto::{FilterQuery, FilterWorklogsRequest, WorklogPageJson};
use crate::error::ApiResult;
use crate::mapper::{
    filter_query_to_request, filter_worklogs_request_to_command, worklog_page_to_json,
};
use crate::routes::controllers::validate::validate_filter;
use crate::state::AppState;

pub async fn filter(
    State(state): State<AppState>,
    Json(mut request): Json<FilterWorklogsRequest>,
) -> ApiResult<Json<WorklogPageJson>> {
    validate_filter(&mut request)?;
    let command = filter_worklogs_request_to_command(request);
    let response = state.filter_worklogs().execute(command).await?;
    Ok(Json(worklog_page_to_json(response)))
}

pub async fn filter_query(
    State(state): State<AppState>,
    Query(query): Query<FilterQuery>,
) -> ApiResult<Json<WorklogPageJson>> {
    let mut request = filter_query_to_request(query);
    validate_filter(&mut request)?;
    let command = filter_worklogs_request_to_command(request);
    let response = state.filter_worklogs().execute(command).await?;
    Ok(Json(worklog_page_to_json(response)))
}
