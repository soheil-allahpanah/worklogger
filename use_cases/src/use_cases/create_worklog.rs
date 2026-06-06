use domain::traits::WorklogRepository;

use crate::dtos::commands::CreateWorklogCommand;
use crate::dtos::responses::CreateWorklogResponse;
use crate::error::UseCaseResult;
use crate::mappers::command_to_worklog;

pub struct CreateWorklogUseCase<R> {
    repository: R,
}

impl<R> CreateWorklogUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: WorklogRepository> CreateWorklogUseCase<R> {
    pub async fn execute(&self, command: CreateWorklogCommand) -> UseCaseResult<CreateWorklogResponse> {
        command.validate()?;
        let worklog = command_to_worklog(command)?;
        let id = worklog.id();
        self.repository.save(&worklog).await?;
        Ok(CreateWorklogResponse::new(id))
    }
}
