use domain::actor::ActorContext;
use domain::traits::{TokenRepository, UserRepository};

use crate::auth::hash_token;
use crate::error::{AuthError, UseCaseResult};

pub struct AuthenticateTokenUseCase<T, U> {
    token_repository: T,
    user_repository: U,
}

impl<T, U> AuthenticateTokenUseCase<T, U> {
    pub fn new(token_repository: T, user_repository: U) -> Self {
        Self {
            token_repository,
            user_repository,
        }
    }
}

impl<T, U> AuthenticateTokenUseCase<T, U>
where
    T: TokenRepository,
    U: UserRepository,
{
    pub async fn execute(&self, raw_token: &str) -> UseCaseResult<ActorContext> {
        let raw_token = raw_token.trim();
        if raw_token.is_empty() {
            return Err(AuthError::InvalidToken.into());
        }

        let token_hash = hash_token(raw_token);
        let token = self
            .token_repository
            .find_by_hash(&token_hash)
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        if !token.is_valid() {
            return Err(AuthError::InvalidToken.into());
        }

        let user = self
            .user_repository
            .get(token.user_id())
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        if !user.is_active() {
            return Err(AuthError::UserInactive.into());
        }

        Ok(ActorContext::new(user.id()))
    }
}
