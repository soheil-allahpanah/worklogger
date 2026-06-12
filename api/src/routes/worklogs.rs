use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use common::filter::{DurationFilter, JalaliDateFilter, ListFilter, TextFilter};
use common::pagination::PagingParams;
use use_cases::{CreateWorklogCommand, DeleteWorklogCommand, FilterWorklogsCommand};
use uuid::Uuid;

use crate::dto::{CreateWorklogJson, CreateWorklogRequest, WorklogPageJson};
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateWorklogRequest>,
) -> ApiResult<(StatusCode, Json<CreateWorklogJson>)> {
    let command: CreateWorklogCommand = body.into();
    let response = state.create_worklog().execute(command).await?;
    Ok((StatusCode::CREATED, Json(response.id.into())))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let command = DeleteWorklogCommand { id };
    state.delete_worklog().execute(command).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn filter(
    State(state): State<AppState>,
    Json(mut command): Json<FilterWorklogsCommand>,
) -> ApiResult<Json<WorklogPageJson>> {
    validate_filter(&mut command)?;
    let response = state.filter_worklogs().execute(command).await?;
    Ok(Json(response.into()))
}

pub async fn filter_query(
    State(state): State<AppState>,
    Query(query): Query<FilterQuery>,
) -> ApiResult<Json<WorklogPageJson>> {
    let mut command = query.into_command();
    validate_filter(&mut command)?;
    let response = state.filter_worklogs().execute(command).await?;
    Ok(Json(response.into()))
}

pub async fn export(
    State(state): State<AppState>,
    Json(mut command): Json<FilterWorklogsCommand>,
) -> ApiResult<impl IntoResponse> {
    validate_filter(&mut command)?;
    let response = state.export_worklogs().execute(command).await?;
    Ok(export_response(response))
}

pub async fn export_query(
    State(state): State<AppState>,
    Query(query): Query<FilterQuery>,
) -> ApiResult<impl IntoResponse> {
    let mut command = query.into_command();
    validate_filter(&mut command)?;
    let response = state.export_worklogs().execute(command).await?;
    Ok(export_response(response))
}

fn validate_filter(command: &mut FilterWorklogsCommand) -> ApiResult<()> {
    command.validate().map_err(|errors| {
        ApiError::bad_request_with_details("invalid filter parameters", errors)
    })
}

fn export_response(response: use_cases::ExportWorklogsResponse) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&response.content_type).unwrap_or(HeaderValue::from_static(
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )),
    );
    let disposition = format!("attachment; filename=\"{}\"", response.filename);
    if let Ok(value) = HeaderValue::from_str(&disposition) {
        headers.insert(header::CONTENT_DISPOSITION, value);
    }
    if let Ok(value) = HeaderValue::from_str(&response.row_count.to_string()) {
        headers.insert("x-row-count", value);
    }
    (headers, response.bytes)
}

#[derive(Debug, serde::Deserialize)]
pub struct FilterQuery {
    pub tags: Option<String>,
    pub exclude_tags: Option<String>,
    pub ids: Option<String>,
    pub exclude_ids: Option<String>,
    pub description: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub duration_from: Option<String>,
    pub duration_to: Option<String>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

impl FilterQuery {
    fn into_command(self) -> FilterWorklogsCommand {
        let tags = list_filter_from_csv(self.tags, self.exclude_tags);
        let ids = uuid_list_filter_from_csv(self.ids, self.exclude_ids);
        let description = self.description.map(|contains| TextFilter {
            contains: Some(contains),
        });
        let date = match (self.date_from, self.date_to) {
            (None, None) => None,
            (from, to) => Some(JalaliDateFilter { from, to }),
        };
        let duration = match (self.duration_from, self.duration_to) {
            (None, None) => None,
            (from, to) => Some(DurationFilter { from, to }),
        };
        let paging = PagingParams {
            page: if self.page == 0 { 1 } else { self.page },
            size: if self.size == 0 { default_page_size() } else { self.size },
        };

        FilterWorklogsCommand {
            tags,
            ids,
            description,
            date,
            duration,
            paging,
        }
    }
}

fn list_filter_from_csv(
    include: Option<String>,
    exclude: Option<String>,
) -> Option<ListFilter<String>> {
    let in_list = include.map(|csv| split_csv(&csv));
    let not_in = exclude.map(|csv| split_csv(&csv));
    if in_list.is_none() && not_in.is_none() {
        None
    } else {
        Some(ListFilter::new(in_list, not_in))
    }
}

fn uuid_list_filter_from_csv(
    include: Option<String>,
    exclude: Option<String>,
) -> Option<ListFilter<Uuid>> {
    let in_list = include.map(|csv| parse_uuid_csv(&csv));
    let not_in = exclude.map(|csv| parse_uuid_csv(&csv));
    if in_list.is_none() && not_in.is_none() {
        None
    } else {
        Some(ListFilter::new(in_list, not_in))
    }
}

fn split_csv(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

fn parse_uuid_csv(raw: &str) -> Vec<Uuid> {
    raw.split(',')
        .filter_map(|s| Uuid::parse_str(s.trim()).ok())
        .collect()
}
