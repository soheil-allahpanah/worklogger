use domain::entities::Worklog;
use common::pagination::PageResult;

/// Output of the create-worklog use case.
pub type FilterWorklogsResponse = PageResult<Worklog>;

