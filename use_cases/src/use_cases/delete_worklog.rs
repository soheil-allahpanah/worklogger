use domain::traits::WorklogRepository;

use crate::dtos::commands::DeleteWorklogCommand;
use crate::error::UseCaseResult;
use domain::value_objects::WorklogId;

pub struct DeleteWorklogUseCase<R> {
    repository: R,
}

impl<R> DeleteWorklogUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: WorklogRepository> DeleteWorklogUseCase<R> {
    pub async fn execute(&self, command: DeleteWorklogCommand) -> UseCaseResult<()> {
        self.repository
            .delete(command.user_id, WorklogId::from_uuid(command.id))
            .await?;
        Ok(())
    }
}
