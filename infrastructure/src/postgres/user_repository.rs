use domain::entities::User;
use domain::traits::{RepositoryError, RepositoryResult, UserRepository};
use domain::value_objects::{Email, UserId, UserName};
use sqlx::PgPool;

use super::user_mapper::row_to_user;
use super::user_row::UserRow;

const USER_SELECT: &str = r#"
    SELECT
        id,
        name,
        email,
        password_hash,
        created_at,
        updated_at,
        disabled_at,
        deleted_at
    FROM users
"#;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn get(&self, id: UserId) -> RepositoryResult<User> {
        let row = sqlx::query_as::<_, UserRow>(&format!("{USER_SELECT} WHERE id = $1"))
            .bind(id.as_uuid())
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| RepositoryError::QueryFailed)?
            .ok_or(RepositoryError::UserNotFound)?;

        row_to_user(row)
    }

    async fn save(&self, user: &User) -> RepositoryResult<()> {
        sqlx::query(
            r#"
            INSERT INTO users (
                id,
                name,
                email,
                password_hash,
                created_at,
                updated_at,
                disabled_at,
                deleted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.name().as_str())
        .bind(user.email().map(|email| email.as_str()))
        .bind(user.password_hash())
        .bind(user.created_at().as_datetime())
        .bind(user.updated_at().as_datetime())
        .bind(user.disabled_at().map(|at| at.as_datetime()))
        .bind(user.deleted_at().map(|at| at.as_datetime()))
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        Ok(())
    }

    async fn update(&self, user: &User) -> RepositoryResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET
                name = $2,
                email = $3,
                password_hash = $4,
                updated_at = $5,
                disabled_at = $6,
                deleted_at = $7
            WHERE id = $1
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.name().as_str())
        .bind(user.email().map(|email| email.as_str()))
        .bind(user.password_hash())
        .bind(user.updated_at().as_datetime())
        .bind(user.disabled_at().map(|at| at.as_datetime()))
        .bind(user.deleted_at().map(|at| at.as_datetime()))
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::UserNotFound);
        }

        Ok(())
    }

    async fn find_by_email(&self, email: &Email) -> RepositoryResult<User> {
        let row = sqlx::query_as::<_, UserRow>(&format!(
            "{USER_SELECT} WHERE email = $1 AND deleted_at IS NULL"
        ))
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| RepositoryError::QueryFailed)?
        .ok_or(RepositoryError::UserNotFound)?;

        row_to_user(row)
    }

    async fn find_by_name(&self, name: &UserName) -> RepositoryResult<Vec<User>> {
        let rows = sqlx::query_as::<_, UserRow>(&format!(
            "{USER_SELECT} WHERE name = $1 AND deleted_at IS NULL"
        ))
        .bind(name.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(|_| RepositoryError::QueryFailed)?;

        rows.into_iter().map(row_to_user).collect()
    }

    async fn disable(&self, id: UserId) -> RepositoryResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET disabled_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL AND disabled_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::UserNotFound);
        }

        Ok(())
    }

    async fn enable(&self, id: UserId) -> RepositoryResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET disabled_at = NULL, updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL AND disabled_at IS NOT NULL
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::UserNotFound);
        }

        Ok(())
    }

    async fn soft_delete(&self, id: UserId) -> RepositoryResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET deleted_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::UserNotFound);
        }

        Ok(())
    }
}
