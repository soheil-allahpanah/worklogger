use chrono::{DateTime, Utc};
use common::pagination::PageResult;
use domain::entities::Worklog;
use domain::value_objects::WorklogId;
use serde::Serialize;
use use_cases::export::worklog_display::{format_description, format_duration_secs, jalali_date_string};

#[derive(Debug, Serialize)]
pub struct CreateWorklogJson {
    pub id: String,
}

impl From<WorklogId> for CreateWorklogJson {
    fn from(id: WorklogId) -> Self {
        Self {
            id: id.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WorklogJson {
    pub id: String,
    pub datetime: DateTime<Utc>,
    pub jalali_date: String,
    pub duration_secs: u64,
    pub duration: String,
    pub tags: Vec<String>,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<Worklog> for WorklogJson {
    fn from(worklog: Worklog) -> Self {
        let duration_secs = worklog.duration().as_std().as_secs();
        Self {
            id: worklog.id().to_string(),
            datetime: worklog.datetime().as_datetime(),
            jalali_date: jalali_date_string(worklog.datetime().as_datetime()),
            duration_secs,
            duration: format_duration_secs(duration_secs),
            tags: worklog
                .tags()
                .iter()
                .map(|tag| tag.as_str().to_string())
                .collect(),
            description: format_description(&worklog),
            created_at: worklog.created_at().as_datetime(),
            updated_at: worklog.updated_at().as_datetime(),
            deleted_at: worklog.deleted_at().map(|ts| ts.as_datetime()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WorklogPageJson {
    pub items: Vec<WorklogJson>,
    pub total_items: u64,
    pub total_pages: u32,
    pub current_page: u32,
    pub page_size: u32,
}

impl From<PageResult<Worklog>> for WorklogPageJson {
    fn from(page: PageResult<Worklog>) -> Self {
        Self {
            items: page.items.into_iter().map(WorklogJson::from).collect(),
            total_items: page.total_items,
            total_pages: page.total_pages,
            current_page: page.current_page,
            page_size: page.page_size,
        }
    }
}
