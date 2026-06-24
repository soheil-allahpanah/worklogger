use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateWorklogJson {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct WorklogJson {
    pub id: String,
    pub user_id: String,
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

#[derive(Debug, Serialize)]
pub struct WorklogPageJson {
    pub items: Vec<WorklogJson>,
    pub total_items: u64,
    pub total_pages: u32,
    pub current_page: u32,
    pub page_size: u32,
}
