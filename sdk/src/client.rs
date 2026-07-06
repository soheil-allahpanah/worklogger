use std::str::FromStr;

use common::pagination::PageResult;
use domain::entities::Worklog;
use domain::value_objects::WorklogId;
use reqwest::{header, Client, RequestBuilder, StatusCode};
use url::Url;
use use_cases::{
    CreateWorklogCommand, CreateWorklogResponse, DeleteWorklogCommand, EditWorklogCommand,
    ExportWorklogsResponse, FilterWorklogsCommand, FilterWorklogsResponse, GetWorklogCommand,
};

use crate::api_types::{CreateWorklogJson, ErrorBody, WorklogJson, WorklogPageJson};
use crate::error::{SdkError, SdkResult};
use crate::mapper::json_to_worklog;

#[derive(Clone)]
pub struct WorkloggerClient {
    http: Client,
    base_url: Url,
    token: String,
}

impl WorkloggerClient {
    pub fn builder() -> crate::WorkloggerClientBuilder {
        crate::WorkloggerClientBuilder::new()
    }

    pub(crate) fn new(base_url: Url, token: String) -> Self {
        Self {
            http: Client::new(),
            base_url,
            token,
        }
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub async fn health(&self) -> SdkResult<()> {
        let url = self.base_url.join("health").map_err(map_url_error)?;
        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(map_network_error)?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(SdkError::Server(format!(
                "health check failed with status {}",
                response.status()
            )))
        }
    }

    pub async fn create_worklog(
        &self,
        command: CreateWorklogCommand,
    ) -> SdkResult<CreateWorklogResponse> {
        command
            .validate()
            .map_err(|e| SdkError::Validation(e.to_string()))?;

        let url = self.base_url.join("worklogs").map_err(map_url_error)?;
        let body = CreateWorklogBody::from(&command);
        let response = self
            .authed(self.http.post(url))
            .json(&body)
            .send()
            .await
            .map_err(map_network_error)?;

        let json: CreateWorklogJson = self.handle_response(response).await?;
        let id = WorklogId::from_str(&json.id)
            .map_err(|e| SdkError::InvalidResponse(e.to_string()))?;
        Ok(CreateWorklogResponse::new(id))
    }

    pub async fn filter_worklogs(
        &self,
        mut command: FilterWorklogsCommand,
    ) -> SdkResult<FilterWorklogsResponse> {
        if let Err(errors) = command.validate() {
            return Err(SdkError::Validation(errors.join("; ")));
        }

        let url = self
            .base_url
            .join("worklogs/filter")
            .map_err(map_url_error)?;
        let body = FilterWorklogsBody::from(&command);
        let response = self
            .authed(self.http.post(url))
            .json(&body)
            .send()
            .await
            .map_err(map_network_error)?;

        let page: WorklogPageJson = self.handle_response(response).await?;
        page_to_result(page)
    }

    pub async fn edit_worklog(&self, command: EditWorklogCommand) -> SdkResult<Worklog> {
        command
            .validate()
            .map_err(|e| SdkError::Validation(e.to_string()))?;

        let url = self
            .base_url
            .join(&format!("worklogs/{}", command.id))
            .map_err(map_url_error)?;
        let body = EditWorklogBody::from(&command);
        let response = self
            .authed(self.http.put(url))
            .json(&body)
            .send()
            .await
            .map_err(map_network_error)?;

        let json: WorklogJson = self.handle_response(response).await?;
        json_to_worklog(json)
    }

    pub async fn get_worklog(&self, command: GetWorklogCommand) -> SdkResult<Worklog> {
        let url = self
            .base_url
            .join(&format!("worklogs/{}", command.id))
            .map_err(map_url_error)?;
        let response = self
            .authed(self.http.get(url))
            .send()
            .await
            .map_err(map_network_error)?;

        let json: WorklogJson = self.handle_response(response).await?;
        json_to_worklog(json)
    }

    pub async fn delete_worklog(&self, command: DeleteWorklogCommand) -> SdkResult<()> {
        let url = self
            .base_url
            .join(&format!("worklogs/{}", command.id))
            .map_err(map_url_error)?;
        let response = self
            .authed(self.http.delete(url))
            .send()
            .await
            .map_err(map_network_error)?;

        self.handle_empty_response(response).await
    }

    pub async fn export_worklogs(
        &self,
        mut command: FilterWorklogsCommand,
    ) -> SdkResult<ExportWorklogsResponse> {
        if let Err(errors) = command.validate() {
            return Err(SdkError::Validation(errors.join("; ")));
        }

        let url = self
            .base_url
            .join("worklogs/export")
            .map_err(map_url_error)?;
        let body = FilterWorklogsBody::from(&command);
        let response = self
            .authed(self.http.post(url))
            .json(&body)
            .send()
            .await
            .map_err(map_network_error)?;

        let status = response.status();
        if status.is_success() {
            let content_type = response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
                .to_owned();
            let filename = response
                .headers()
                .get(header::CONTENT_DISPOSITION)
                .and_then(|v| v.to_str().ok())
                .and_then(parse_filename)
                .unwrap_or_else(default_export_filename);
            let row_count = response
                .headers()
                .get("x-row-count")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            let bytes = response
                .bytes()
                .await
                .map_err(map_network_error)?
                .to_vec();
            return Ok(ExportWorklogsResponse::new(
                bytes,
                filename,
                content_type,
                row_count,
            ));
        }

        Err(self.error_from_response(response).await?)
    }

    fn authed(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.bearer_auth(&self.token)
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> SdkResult<T> {
        if response.status().is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| SdkError::InvalidResponse(e.to_string()))
        } else {
            Err(self.error_from_response(response).await?)
        }
    }

    async fn handle_empty_response(&self, response: reqwest::Response) -> SdkResult<()> {
        if response.status().is_success() {
            Ok(())
        } else {
            Err(self.error_from_response(response).await?)
        }
    }

    async fn error_from_response(&self, response: reqwest::Response) -> SdkResult<SdkError> {
        let status = response.status();
        let body = response
            .json::<ErrorBody>()
            .await
            .unwrap_or(ErrorBody {
                error: status.to_string(),
                details: None,
            });

        let message = if let Some(details) = body.details.filter(|d| !d.is_empty()) {
            format!("{}: {}", body.error, details.join("; "))
        } else {
            body.error
        };

        Err(match status {
            StatusCode::UNAUTHORIZED => SdkError::Unauthorized(message),
            StatusCode::FORBIDDEN => SdkError::Forbidden(message),
            StatusCode::NOT_FOUND => SdkError::NotFound(message),
            StatusCode::BAD_REQUEST => SdkError::BadRequest(message),
            s if s.is_server_error() => SdkError::Server(message),
            _ => SdkError::BadRequest(message),
        })
    }
}

fn page_to_result(page: WorklogPageJson) -> SdkResult<FilterWorklogsResponse> {
    let items = page
        .items
        .into_iter()
        .map(json_to_worklog)
        .collect::<SdkResult<Vec<_>>>()?;
    Ok(FilterWorklogsResponse {
        page: PageResult::new(
            items,
            page.total_items,
            page.current_page,
            page.page_size,
        ),
        statistics: use_cases::WorklogFilterStatistics {
            total_duration_secs: page.statistics.total_duration_secs,
            days_worked: page.statistics.days_worked,
        },
    })
}

#[derive(serde::Serialize)]
struct CreateWorklogBody {
    jalali_date: Option<String>,
    duration_secs: u64,
    tags: Vec<String>,
    description: String,
}

impl From<&CreateWorklogCommand> for CreateWorklogBody {
    fn from(command: &CreateWorklogCommand) -> Self {
        Self {
            jalali_date: command.jalali_date.clone(),
            duration_secs: command.duration_secs,
            tags: command.tags.clone(),
            description: command.description.clone(),
        }
    }
}

#[derive(serde::Serialize)]
struct EditWorklogBody {
    jalali_date: Option<String>,
    duration_secs: u64,
    tags: Vec<String>,
    description: String,
}

impl From<&EditWorklogCommand> for EditWorklogBody {
    fn from(command: &EditWorklogCommand) -> Self {
        Self {
            jalali_date: command.jalali_date.clone(),
            duration_secs: command.duration_secs,
            tags: command.tags.clone(),
            description: command.description.clone(),
        }
    }
}

#[derive(serde::Serialize)]
struct FilterWorklogsBody {
    tags: Option<common::filter::ListFilter<String>>,
    ids: Option<common::filter::ListFilter<uuid::Uuid>>,
    description: Option<common::filter::TextFilter>,
    date: Option<common::filter::JalaliDateFilter>,
    duration: Option<common::filter::DurationFilter>,
    paging: common::pagination::PagingParams,
}

impl From<&FilterWorklogsCommand> for FilterWorklogsBody {
    fn from(command: &FilterWorklogsCommand) -> Self {
        Self {
            tags: command.tags.clone(),
            ids: command.ids.clone(),
            description: command.description.clone(),
            date: command.date.clone(),
            duration: command.duration.clone(),
            paging: command.paging.clone(),
        }
    }
}

fn map_network_error(err: reqwest::Error) -> SdkError {
    SdkError::Network(err.to_string())
}

fn map_url_error(err: url::ParseError) -> SdkError {
    SdkError::Config(format!("invalid URL: {err}"))
}

fn parse_filename(value: &str) -> Option<String> {
    value
        .split("filename=")
        .nth(1)
        .map(|name| name.trim_matches('"').to_owned())
}

fn default_export_filename() -> String {
    let now = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    format!("worklogs_{now}.xlsx")
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::value_objects::UserId;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_client(base: &str, token: &str) -> WorkloggerClient {
        WorkloggerClient::new(Url::parse(base).unwrap(), token.to_owned())
    }

    #[tokio::test]
    async fn health_succeeds_without_auth() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
            .mount(&server)
            .await;

        let client = test_client(&server.uri(), "wl_test");
        client.health().await.unwrap();
    }

    #[tokio::test]
    async fn filter_worklogs_requires_bearer_token() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/worklogs/filter"))
            .and(header("authorization", "Bearer wl_test"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [],
                "total_items": 0,
                "total_pages": 0,
                "current_page": 1,
                "page_size": 20,
                "statistics": {
                    "total_duration_secs": 0,
                    "days_worked": 0
                }
            })))
            .mount(&server)
            .await;

        let client = test_client(&server.uri(), "wl_test");
        let command = FilterWorklogsCommand {
            user_id: UserId::generate(),
            tags: None,
            ids: None,
            description: None,
            date: None,
            duration: None,
            paging: common::pagination::PagingParams::default(),
        };
        let response = client.filter_worklogs(command).await.unwrap();
        assert!(response.page.items.is_empty());
    }

    #[tokio::test]
    async fn unauthorized_maps_to_sdk_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/worklogs/550e8400-e29b-41d4-a716-446655440000"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "invalid or expired token"
            })))
            .mount(&server)
            .await;

        let client = test_client(&server.uri(), "bad");
        let err = client
            .get_worklog(GetWorklogCommand {
                user_id: UserId::generate(),
                id: uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            })
            .await
            .unwrap_err();

        assert!(matches!(err, SdkError::Unauthorized(_)));
    }
}
