use crate::value_objects::WorklogDuration;

/// Resolved duration bounds for filtering worklogs.
#[derive(Debug, Clone)]
pub struct WorklogDurationFilter {
    pub from: Option<WorklogDuration>,
    pub to: Option<WorklogDuration>,
}
