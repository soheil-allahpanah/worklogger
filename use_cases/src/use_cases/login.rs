use domain::traits::{RefreshTokenRepository, UserRepository};
use domain::value_objects::{Email, UserName};

use crate::auth::{verify_password, JwtConfig};
use crate::dtos::commands::LoginCommand;
use crate::dtos::responses::AuthTokensResponse;
use crate::error::{AuthError, UseCaseResult};

use super::auth_tokens::issue_auth_tokens;

pub struct LoginUseCase<R, U> {
    refresh_token_repository: R,
    user_repository: U,
    jwt_config: JwtConfig,
}

impl<R, U> LoginUseCase<R, U> {
    pub fn new(refresh_token_repository: R, user_repository: U, jwt_config: JwtConfig) -> Self {
        Self {
            refresh_token_repository,
            user_repository,
            jwt_config,
        }
    }
}

impl<R, U> LoginUseCase<R, U>
where
    R: RefreshTokenRepository,
    U: UserRepository,
{
    pub async fn execute(&self, command: LoginCommand) -> UseCaseResult<AuthTokensResponse> {
        let login = command.login.trim();
        if login.is_empty() || command.password.is_empty() {
            return Err(AuthError::InvalidCredentials.into());
        }

        let user = self.find_user_by_login(login).await?;
        if !user.is_active() {
            return Err(AuthError::InvalidCredentials.into());
        }

        let password_hash = user
            .password_hash()
            .ok_or(AuthError::InvalidCredentials)?;
        verify_password(&command.password, password_hash)?;

        issue_auth_tokens(
            &self.refresh_token_repository,
            &self.jwt_config,
            user.id(),
        )
        .await
    }

    async fn find_user_by_login(
        &self,
        login: &str,
    ) -> UseCaseResult<domain::entities::User> {
        if let Ok(email) = Email::try_new(login) {
            if let Ok(user) = self.user_repository.find_by_email(&email).await {
                return Ok(user);
            }
        }

        let name = UserName::try_new(login).map_err(|_| AuthError::InvalidCredentials)?;
        let users = self
            .user_repository
            .find_by_name(&name)
            .await
            .map_err(|_| AuthError::InvalidCredentials)?;

        match users.len() {
            1 => Ok(users.into_iter().next().expect("one user")),
            _ => Err(AuthError::InvalidCredentials.into()),
        }
    }
}
