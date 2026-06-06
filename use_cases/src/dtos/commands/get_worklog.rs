use uuid::Uuid;

/// Input for the delete-worklog use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWorklogCommand {
    /// Worklog ID.
    pub id: Uuid,
}
