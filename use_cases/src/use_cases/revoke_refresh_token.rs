use domain::traits::RefreshTokenRepository;

use crate::auth::hash_token;
use crate::dtos::commands::RevokeRefreshTokenCommand;
use crate::error::{AuthError, UseCaseResult};

pub struct RevokeRefreshTokenUseCase<R> {
    refresh_token_repository: R,
}

impl<R> RevokeRefreshTokenUseCase<R> {
    pub fn new(refresh_token_repository: R) -> Self {
        Self {
            refresh_token_repository,
        }
    }
}

impl<R: RefreshTokenRepository> RevokeRefreshTokenUseCase<R> {
    pub async fn execute(&self, command: RevokeRefreshTokenCommand) -> UseCaseResult<()> {
        let raw_token = command.refresh_token.trim();
        if raw_token.is_empty() {
            return Err(AuthError::InvalidToken.into());
        }

        let token_hash = hash_token(raw_token);
        let refresh_token = self
            .refresh_token_repository
            .find_by_hash(&token_hash)
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        if refresh_token.is_revoked() {
            return Ok(());
        }

        self.refresh_token_repository
            .revoke(refresh_token.id())
            .await?;

        Ok(())
    }
}
