use domain::traits::UserRepository;

use crate::dtos::commands::EnableUserCommand;
use crate::error::UseCaseResult;

pub struct EnableUserUseCase<R> {
    user_repository: R,
}

impl<R> EnableUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

impl<R: UserRepository> EnableUserUseCase<R> {
    pub async fn execute(&self, command: EnableUserCommand) -> UseCaseResult<()> {
        self.user_repository.enable(command.user_id).await?;
        Ok(())
    }
}
