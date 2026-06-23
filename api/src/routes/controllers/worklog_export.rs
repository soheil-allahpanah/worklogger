use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

use crate::dto::{FilterQuery, FilterWorklogsRequest};
use crate::error::ApiResult;
use crate::mapper::{
    export_response_to_http, filter_query_to_request, filter_worklogs_request_to_command,
};
use crate::routes::controllers::validate::validate_filter;
use crate::state::AppState;

pub async fn export(
    State(state): State<AppState>,
    Json(mut request): Json<FilterWorklogsRequest>,
) -> ApiResult<impl IntoResponse> {
    validate_filter(&mut request)?;
    let command = filter_worklogs_request_to_command(request);
    let response = state.export_worklogs().execute(command).await?;
    Ok(export_response_to_http(response))
}

pub async fn export_query(
    State(state): State<AppState>,
    Query(query): Query<FilterQuery>,
) -> ApiResult<impl IntoResponse> {
    let mut request = filter_query_to_request(query);
    validate_filter(&mut request)?;
    let command = filter_worklogs_request_to_command(request);
    let response = state.export_worklogs().execute(command).await?;
    Ok(export_response_to_http(response))
}
