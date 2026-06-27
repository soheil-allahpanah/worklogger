use crate::entities::RefreshToken;
use crate::value_objects::TokenId;

use super::repository_error::RepositoryResult;

/// Persistence port for JWT refresh tokens.
pub trait RefreshTokenRepository {
    async fn save(&self, token: &RefreshToken) -> RepositoryResult<()>;
    async fn find_by_hash(&self, token_hash: &[u8]) -> RepositoryResult<RefreshToken>;
    async fn revoke(&self, id: TokenId) -> RepositoryResult<()>;
}

impl<R: RefreshTokenRepository> RefreshTokenRepository for std::sync::Arc<R> {
    async fn save(&self, token: &RefreshToken) -> RepositoryResult<()> {
        self.as_ref().save(token).await
    }

    async fn find_by_hash(&self, token_hash: &[u8]) -> RepositoryResult<RefreshToken> {
        self.as_ref().find_by_hash(token_hash).await
    }

    async fn revoke(&self, id: TokenId) -> RepositoryResult<()> {
        self.as_ref().revoke(id).await
    }
}
