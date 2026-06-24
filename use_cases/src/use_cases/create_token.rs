use domain::entities::ApiToken;
use domain::traits::{TokenRepository, UserRepository};

use crate::auth::{generate_raw_token, hash_token};
use crate::dtos::commands::CreateTokenCommand;
use crate::dtos::responses::CreateTokenResponse;
use crate::error::{AuthError, UseCaseResult};

pub struct CreateTokenUseCase<T, U> {
    token_repository: T,
    user_repository: U,
}

impl<T, U> CreateTokenUseCase<T, U> {
    pub fn new(token_repository: T, user_repository: U) -> Self {
        Self {
            token_repository,
            user_repository,
        }
    }
}

impl<T, U> CreateTokenUseCase<T, U>
where
    T: TokenRepository,
    U: UserRepository,
{
    pub async fn execute(&self, command: CreateTokenCommand) -> UseCaseResult<CreateTokenResponse> {
        let user = self.user_repository.get(command.user_id).await?;
        if !user.is_active() {
            return Err(AuthError::UserInactive.into());
        }

        let raw_token = generate_raw_token();
        let token_hash = hash_token(&raw_token);
        let token = ApiToken::create(command.user_id, token_hash, command.label);
        let id = token.id();
        self.token_repository.save(&token).await?;
        Ok(CreateTokenResponse::new(id, raw_token))
    }
}
