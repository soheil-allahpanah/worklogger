use domain::entities::User;
use domain::traits::UserRepository;
use domain::value_objects::{Email, UserName};

use crate::dtos::commands::CreateUserCommand;
use crate::dtos::responses::CreateUserResponse;
use crate::error::UseCaseResult;

pub struct CreateUserUseCase<R> {
    user_repository: R,
}

impl<R> CreateUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub async fn execute(&self, command: CreateUserCommand) -> UseCaseResult<CreateUserResponse> {
        let name = UserName::try_new(command.name)?;
        let email = command
            .email
            .map(|value| Email::try_new(value))
            .transpose()?;

        let user = User::create(name, email);
        let id = user.id();
        let name = user.name().as_str().to_owned();
        self.user_repository.save(&user).await?;
        Ok(CreateUserResponse::new(id, name))
    }
}
