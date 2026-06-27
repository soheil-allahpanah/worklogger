use domain::traits::UserRepository;

use crate::auth::{validate_access_token, JwtConfig};
use crate::error::{AuthError, UseCaseResult};

pub struct AuthenticateJwtUseCase<U> {
    user_repository: U,
    jwt_config: JwtConfig,
}

impl<U> AuthenticateJwtUseCase<U> {
    pub fn new(user_repository: U, jwt_config: JwtConfig) -> Self {
        Self {
            user_repository,
            jwt_config,
        }
    }
}

impl<U: UserRepository> AuthenticateJwtUseCase<U> {
    pub async fn execute(
        &self,
        raw_token: &str,
    ) -> UseCaseResult<domain::actor::ActorContext> {
        let raw_token = raw_token.trim();
        if raw_token.is_empty() {
            return Err(AuthError::InvalidToken.into());
        }

        let user_id = validate_access_token(&self.jwt_config, raw_token)?;
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        if !user.is_active() {
            return Err(AuthError::UserInactive.into());
        }

        Ok(domain::actor::ActorContext::new(user.id()))
    }
}
