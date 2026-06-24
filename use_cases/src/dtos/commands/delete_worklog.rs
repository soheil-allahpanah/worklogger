use uuid::Uuid;

use domain::value_objects::UserId;

/// Input for the delete-worklog use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteWorklogCommand {
    pub user_id: UserId,
    /// Worklog ID.
    pub id: Uuid,
}
