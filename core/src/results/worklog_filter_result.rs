use crate::entities::Worklog;

/// Paginated filter result with aggregate statistics over the full filtered set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorklogFilterResult {
    pub items: Vec<Worklog>,
    pub total_items: u64,
    pub total_duration_secs: u64,
    pub days_worked: u64,
}
