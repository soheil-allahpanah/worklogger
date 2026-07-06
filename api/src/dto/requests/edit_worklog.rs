use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EditWorklogRequest {
    /// Jalali calendar date (`YYYY-MM-DD` or `YYYY/MM/DD`). When omitted, today (Asia/Tehran) is used.
    pub jalali_date: Option<String>,
    /// Work session length in seconds (must be > 0 and < 86_400).
    pub duration_secs: u64,
    pub tags: Vec<String>,
    pub description: String,
}
