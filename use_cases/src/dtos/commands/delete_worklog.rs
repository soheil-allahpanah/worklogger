use uuid::Uuid;

/// Input for the delete-worklog use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteWorklogCommand {
    /// Worklog ID.
    pub id: Uuid,
}
