use domain::entities::ApiToken;
use domain::traits::{RepositoryError, RepositoryResult, TokenRepository};
use domain::value_objects::TokenId;
use sqlx::PgPool;

use super::user_mapper::row_to_token;
use super::user_row::TokenRow;

const TOKEN_SELECT: &str = r#"
    SELECT
        id,
        user_id,
        token_hash,
        label,
        created_at,
        expires_at,
        revoked_at
    FROM api_tokens
"#;

#[derive(Clone)]
pub struct PostgresTokenRepository {
    pool: PgPool,
}

impl PostgresTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl TokenRepository for PostgresTokenRepository {
    async fn save(&self, token: &ApiToken) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            INSERT INTO api_tokens (
                id,
                user_id,
                token_hash,
                label,
                created_at,
                expires_at,
                revoked_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(token.id().as_uuid())
        .bind(token.user_id().as_uuid())
        .bind(token.token_hash())
        .bind(token.label())
        .bind(token.created_at().as_datetime())
        .bind(token.expires_at())
        .bind(token.revoked_at().map(|at| at.as_datetime()))
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        Ok(())
    }

    async fn find_by_hash(&self, token_hash: &[u8]) -> RepositoryResult<ApiToken> {
        let row = sqlx::query_as::<_, TokenRow>(&format!(
            "{TOKEN_SELECT} WHERE token_hash = $1"
        ))
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| RepositoryError::QueryFailed)?
        .ok_or(RepositoryError::TokenNotFound)?;

        row_to_token(row)
    }

    async fn revoke(&self, id: TokenId) -> RepositoryResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE api_tokens
            SET revoked_at = NOW()
            WHERE id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::TokenNotFound);
        }

        Ok(())
    }
}
