use domain::traits::UserRepository;

use crate::dtos::commands::DeleteUserCommand;
use crate::error::UseCaseResult;

pub struct DeleteUserUseCase<R> {
    user_repository: R,
}

impl<R> DeleteUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub async fn execute(&self, command: DeleteUserCommand) -> UseCaseResult<()> {
        self.user_repository.soft_delete(command.user_id).await?;
        Ok(())
    }
}
