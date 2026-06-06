use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// Row shape shared by compile-time-checked SELECT queries.
#[derive(Debug, FromRow)]
pub struct WorklogRow {
    pub id: Uuid,
    pub datetime: DateTime<Utc>,
    pub duration_secs: i64,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
