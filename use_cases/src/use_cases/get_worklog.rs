use domain::traits::WorklogRepository;

use crate::dtos::commands::GetWorklogCommand;
use crate::error::UseCaseResult;
use domain::entities::Worklog;
use domain::value_objects::WorklogId;
pub struct GetWorklogUseCase<R> {
    repository: R,
}

impl<R> GetWorklogUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: WorklogRepository> GetWorklogUseCase<R> {
    pub async fn execute(&self, command: GetWorklogCommand) -> UseCaseResult<Worklog> {
        let worklog = self.repository.get(WorklogId::from_uuid(command.id)).await?;
        Ok(worklog)
    }
}
