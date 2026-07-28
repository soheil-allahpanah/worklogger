/// Per-tag aggregate over the full filtered worklog set (not a page).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorklogTagStat {
    pub tag: String,
    pub duration_secs: u64,
    /// Distinct Tehran calendar days that include this tag.
    pub days_worked: u64,
    pub worklog_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorklogTagStatsResult {
    pub tags: Vec<WorklogTagStat>,
}
