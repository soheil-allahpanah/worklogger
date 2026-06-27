use domain::traits::{RefreshTokenRepository, UserRepository};

use crate::auth::{hash_token, JwtConfig};
use crate::dtos::commands::RefreshAccessTokenCommand;
use crate::dtos::responses::AuthTokensResponse;
use crate::error::{AuthError, UseCaseResult};

use super::auth_tokens::issue_auth_tokens;

pub struct RefreshAccessTokenUseCase<R, U> {
    refresh_token_repository: R,
    user_repository: U,
    jwt_config: JwtConfig,
}

impl<R, U> RefreshAccessTokenUseCase<R, U> {
    pub fn new(refresh_token_repository: R, user_repository: U, jwt_config: JwtConfig) -> Self {
        Self {
            refresh_token_repository,
            user_repository,
            jwt_config,
        }
    }
}

impl<R, U> RefreshAccessTokenUseCase<R, U>
where
    R: RefreshTokenRepository,
    U: UserRepository,
{
    pub async fn execute(
        &self,
        command: RefreshAccessTokenCommand,
    ) -> UseCaseResult<AuthTokensResponse> {
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

        if !refresh_token.is_valid() {
            return Err(AuthError::InvalidToken.into());
        }

        let user = self
            .user_repository
            .get(refresh_token.user_id())
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        if !user.is_active() {
            return Err(AuthError::UserInactive.into());
        }

        self.refresh_token_repository
            .revoke(refresh_token.id())
            .await?;

        issue_auth_tokens(
            &self.refresh_token_repository,
            &self.jwt_config,
            user.id(),
        )
        .await
    }
}
