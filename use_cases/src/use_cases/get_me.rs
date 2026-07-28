use domain::actor::ActorContext;
use domain::traits::UserRepository;

use crate::dtos::responses::MeResponse;
use crate::error::{AuthError, UseCaseResult};

pub struct GetMeUseCase<U> {
    user_repository: U,
}

impl<U> GetMeUseCase<U> {
    pub fn new(user_repository: U) -> Self {
        Self { user_repository }
    }
}

impl<U: UserRepository> GetMeUseCase<U> {
    pub async fn execute(&self, actor: ActorContext) -> UseCaseResult<MeResponse> {
        let user = self.user_repository.get(actor.user_id()).await?;

        if !user.is_active() {
            return Err(AuthError::UserInactive.into());
        }

        Ok(MeResponse::new(
            user.id().to_string(),
            user.name().as_str(),
            user.email().map(|email| email.as_str().to_owned()),
        ))
    }
}
