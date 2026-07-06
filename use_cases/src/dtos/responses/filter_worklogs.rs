use common::pagination::PageResult;
use domain::entities::Worklog;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorklogFilterStatistics {
    pub total_duration_secs: u64,
    pub days_worked: u64,
}

/// Output of the filter-worklogs use case.
#[derive(Debug, Clone)]
pub struct FilterWorklogsResponse {
    pub page: PageResult<Worklog>,
    pub statistics: WorklogFilterStatistics,
}
