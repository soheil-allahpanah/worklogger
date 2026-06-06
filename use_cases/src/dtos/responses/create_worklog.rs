use domain::value_objects::WorklogId;

/// Output of the create-worklog use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateWorklogResponse {
    pub id: WorklogId,
}

impl CreateWorklogResponse {
    pub fn new(id: WorklogId) -> Self {
        Self { id }
    }
}
