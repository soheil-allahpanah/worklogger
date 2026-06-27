use domain::entities::RefreshToken;
use domain::traits::{RefreshTokenRepository, RepositoryError, RepositoryResult};
use domain::value_objects::TokenId;
use sqlx::PgPool;

use super::user_mapper::row_to_refresh_token;
use super::user_row::RefreshTokenRow;

const REFRESH_TOKEN_SELECT: &str = r#"
    SELECT
        id,
        user_id,
        token_hash,
        created_at,
        expires_at,
        revoked_at
    FROM refresh_tokens
"#;

#[derive(Clone)]
pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
}

impl PostgresRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn save(&self, token: &RefreshToken) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (
                id,
                user_id,
                token_hash,
                created_at,
                expires_at,
                revoked_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(token.id().as_uuid())
        .bind(token.user_id().as_uuid())
        .bind(token.token_hash())
        .bind(token.created_at().as_datetime())
        .bind(token.expires_at())
        .bind(token.revoked_at().map(|at| at.as_datetime()))
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        Ok(())
    }

    async fn find_by_hash(&self, token_hash: &[u8]) -> RepositoryResult<RefreshToken> {
        let row = sqlx::query_as::<_, RefreshTokenRow>(&format!(
            "{REFRESH_TOKEN_SELECT} WHERE token_hash = $1"
        ))
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| RepositoryError::QueryFailed)?
        .ok_or(RepositoryError::TokenNotFound)?;

        row_to_refresh_token(row)
    }

    async fn revoke(&self, id: TokenId) -> RepositoryResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE refresh_tokens
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
