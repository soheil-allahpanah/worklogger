use domain::traits::UserRepository;

use crate::dtos::commands::DisableUserCommand;
use crate::error::UseCaseResult;

pub struct DisableUserUseCase<R> {
    user_repository: R,
}

impl<R> DisableUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

impl<R: UserRepository> DisableUserUseCase<R> {
    pub async fn execute(&self, command: DisableUserCommand) -> UseCaseResult<()> {
        self.user_repository.disable(command.user_id).await?;
        Ok(())
    }
}
