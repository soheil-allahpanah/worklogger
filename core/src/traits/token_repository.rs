use crate::entities::ApiToken;
use crate::value_objects::TokenId;

use super::repository_error::RepositoryResult;

/// Persistence port for API device tokens.
pub trait TokenRepository {
    async fn save(&self, token: &ApiToken) -> RepositoryResult<()>;
    async fn find_by_hash(&self, token_hash: &[u8]) -> RepositoryResult<ApiToken>;
    async fn revoke(&self, id: TokenId) -> RepositoryResult<()>;
}

impl<R: TokenRepository> TokenRepository for std::sync::Arc<R> {
    async fn save(&self, token: &ApiToken) -> RepositoryResult<()> {
        self.as_ref().save(token).await
    }

    async fn find_by_hash(&self, token_hash: &[u8]) -> RepositoryResult<ApiToken> {
        self.as_ref().find_by_hash(token_hash).await
    }

    async fn revoke(&self, id: TokenId) -> RepositoryResult<()> {
        self.as_ref().revoke(id).await
    }
}
