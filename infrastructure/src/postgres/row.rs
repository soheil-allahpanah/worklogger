use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// Row shape shared by compile-time-checked SELECT queries.
#[derive(Debug, FromRow)]
pub struct WorklogRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub datetime: DateTime<Utc>,
    pub duration_secs: i64,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Row shape for paginated filter queries that include aggregate statistics.
#[derive(Debug, FromRow)]
pub struct WorklogFilterRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub datetime: DateTime<Utc>,
    pub duration_secs: i64,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub total_count: i64,
    pub total_duration_secs: i64,
    pub days_worked: i64,
}
