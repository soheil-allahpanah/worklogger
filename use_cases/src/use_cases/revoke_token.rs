use domain::traits::TokenRepository;

use crate::dtos::commands::RevokeTokenCommand;
use crate::error::UseCaseResult;

pub struct RevokeTokenUseCase<R> {
    token_repository: R,
}

impl<R> RevokeTokenUseCase<R> {
    pub fn new(token_repository: R) -> Self {
        Self { token_repository }
    }
}

impl<R: TokenRepository> RevokeTokenUseCase<R> {
    pub async fn execute(&self, command: RevokeTokenCommand) -> UseCaseResult<()> {
        self.token_repository.revoke(command.token_id).await?;
        Ok(())
    }
}
