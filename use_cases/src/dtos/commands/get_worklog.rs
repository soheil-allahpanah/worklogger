use uuid::Uuid;

use domain::value_objects::UserId;

/// Input for the get-worklog use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWorklogCommand {
    pub user_id: UserId,
    /// Worklog ID.
    pub id: Uuid,
}
