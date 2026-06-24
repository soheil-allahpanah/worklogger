use crate::entities::User;
use crate::value_objects::UserId;

use super::repository_error::RepositoryResult;

/// Persistence port for the `User` aggregate.
pub trait UserRepository {
    async fn get(&self, id: UserId) -> RepositoryResult<User>;
    async fn save(&self, user: &User) -> RepositoryResult<()>;
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
