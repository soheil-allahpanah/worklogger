use crate::entities::User;
use crate::value_objects::{Email, UserId, UserName};

use super::repository_error::RepositoryResult;

/// Persistence port for the `User` aggregate.
pub trait UserRepository {
    async fn get(&self, id: UserId) -> RepositoryResult<User>;
    async fn save(&self, user: &User) -> RepositoryResult<()>;
    async fn update(&self, user: &User) -> RepositoryResult<()>;
    async fn find_by_email(&self, email: &Email) -> RepositoryResult<User>;
    async fn find_by_name(&self, name: &UserName) -> RepositoryResult<Vec<User>>;
    async fn disable(&self, id: UserId) -> RepositoryResult<()>;
    async fn enable(&self, id: UserId) -> RepositoryResult<()>;
    async fn soft_delete(&self, id: UserId) -> RepositoryResult<()>;
}

impl<R: UserRepository> UserRepository for std::sync::Arc<R> {
    async fn get(&self, id: UserId) -> RepositoryResult<User> {
        self.as_ref().get(id).await
    }

    async fn save(&self, user: &User) -> RepositoryResult<()> {
        self.as_ref().save(user).await
    }

    async fn update(&self, user: &User) -> RepositoryResult<()> {
        self.as_ref().update(user).await
    }

    async fn find_by_email(&self, email: &Email) -> RepositoryResult<User> {
        self.as_ref().find_by_email(email).await
    }

    async fn find_by_name(&self, name: &UserName) -> RepositoryResult<Vec<User>> {
        self.as_ref().find_by_name(name).await
    }

    async fn disable(&self, id: UserId) -> RepositoryResult<()> {
        self.as_ref().disable(id).await
    }

    async fn enable(&self, id: UserId) -> RepositoryResult<()> {
        self.as_ref().enable(id).await
    }

    async fn soft_delete(&self, id: UserId) -> RepositoryResult<()> {
        self.as_ref().soft_delete(id).await
    }
}
