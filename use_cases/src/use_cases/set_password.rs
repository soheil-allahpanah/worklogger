use domain::traits::UserRepository;

use crate::auth::hash_password;
use crate::dtos::commands::SetPasswordCommand;
use crate::error::{AuthError, UseCaseResult};

pub struct SetPasswordUseCase<R> {
    user_repository: R,
}

impl<R> SetPasswordUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

impl<R: UserRepository> SetPasswordUseCase<R> {
    pub async fn execute(&self, command: SetPasswordCommand) -> UseCaseResult<()> {
        if command.password.is_empty() {
            return Err(AuthError::InvalidCredentials.into());
        }

        let mut user = self.user_repository.get(command.user_id).await?;
        if !user.is_active() {
            return Err(AuthError::UserInactive.into());
        }

        let password_hash = hash_password(&command.password)?;
        user.set_password(password_hash)?;
        self.user_repository.update(&user).await?;

        Ok(())
    }
}
