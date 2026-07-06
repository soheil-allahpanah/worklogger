use domain::entities::Worklog;
use domain::traits::WorklogRepository;
use domain::value_objects::WorklogId;

use crate::dtos::commands::EditWorklogCommand;
use crate::error::UseCaseResult;
use crate::mappers::apply_edit_command;

pub struct EditWorklogUseCase<R> {
    repository: R,
}

impl<R> EditWorklogUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: WorklogRepository> EditWorklogUseCase<R> {
    pub async fn execute(&self, command: EditWorklogCommand) -> UseCaseResult<Worklog> {
        command.validate()?;
        let mut worklog = self
            .repository
            .get(command.user_id, WorklogId::from_uuid(command.id))
            .await?;
        apply_edit_command(&mut worklog, command)?;
        self.repository.update(&worklog).await?;
        Ok(worklog)
    }
}
