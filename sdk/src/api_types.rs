use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorBody {
    pub error: String,
    #[serde(default)]
    pub details: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorklogJson {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct WorklogJson {
    pub id: String,
    pub user_id: String,
    pub datetime: chrono::DateTime<chrono::Utc>,
    pub jalali_date: String,
    pub duration_secs: u64,
    pub duration: String,
    pub tags: Vec<String>,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct WorklogPageJson {
    pub items: Vec<WorklogJson>,
    pub total_items: u64,
    pub total_pages: u32,
    pub current_page: u32,
    pub page_size: u32,
}
