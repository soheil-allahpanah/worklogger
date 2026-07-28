#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagStatResponse {
    pub tag: String,
    pub duration_secs: u64,
    pub days_worked: u64,
    pub worklog_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagStatsResponse {
    pub tags: Vec<TagStatResponse>,
}
