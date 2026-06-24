use domain::criteria::WorklogFilterCriteria;
use domain::entities::Worklog;
use domain::traits::{RepositoryError, RepositoryResult, WorklogRepository};
use domain::value_objects::{UserId, WorklogId};
use sqlx::PgPool;

use super::filter_binds::FilterBinds;
use super::mapper::{duration_secs, row_to_worklog};
use super::row::WorklogRow;

const WORKLOG_SELECT: &str = r#"
    SELECT
        id,
        user_id,
        datetime,
        EXTRACT(EPOCH FROM duration)::bigint AS duration_secs,
        tags,
        description,
        created_at,
        updated_at,
        deleted_at
    FROM worklogs
"#;

/// PostgreSQL implementation of [`WorklogRepository`].
#[derive(Clone)]
pub struct PostgresWorklogRepository {
    pool: PgPool,
}

impl PostgresWorklogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl WorklogRepository for PostgresWorklogRepository {
    async fn get(&self, user_id: UserId, id: WorklogId) -> RepositoryResult<Worklog> {
        let row = sqlx::query_as::<_, WorklogRow>(&format!(
            "{WORKLOG_SELECT} WHERE id = $1 AND user_id = $2"
        ))
        .bind(id.as_uuid())
        .bind(user_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| RepositoryError::QueryFailed)?
        .ok_or(RepositoryError::NotFound)?;

        row_to_worklog(row)
    }

    async fn save(&self, worklog: &Worklog) -> RepositoryResult<()> {
        let duration = duration_secs(worklog.duration())?;
        let tags: Vec<String> = worklog
            .tags()
            .iter()
            .map(|tag| tag.as_str().to_owned())
            .collect();
        let description = worklog.description().map(|d| d.as_str().to_owned());

        sqlx::query(
            r#"
            INSERT INTO worklogs (
                id,
                user_id,
                datetime,
                duration,
                tags,
                description,
                created_at,
                updated_at,
                deleted_at
            )
            VALUES ($1, $2, $3, make_interval(secs => $4), $5, $6, $7, $8, $9)
            "#,
        )
        .bind(worklog.id().as_uuid())
        .bind(worklog.user_id().as_uuid())
        .bind(worklog.datetime().as_datetime())
        .bind(duration)
        .bind(&tags)
        .bind(description)
        .bind(worklog.created_at().as_datetime())
        .bind(worklog.updated_at().as_datetime())
        .bind(worklog.deleted_at().map(|d| d.as_datetime()))
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        Ok(())
    }

    async fn filter(&self, criteria: &WorklogFilterCriteria) -> RepositoryResult<Vec<Worklog>> {
        let binds = FilterBinds::from(criteria);

        let rows = sqlx::query_as::<_, WorklogRow>(
            r#"
            SELECT
                id,
                user_id,
                datetime,
                EXTRACT(EPOCH FROM duration)::bigint AS duration_secs,
                tags,
                description,
                created_at,
                updated_at,
                deleted_at
            FROM worklogs
            WHERE deleted_at IS NULL
              AND user_id = $1
              AND ($2::uuid[] IS NULL OR id = ANY($2))
              AND ($3::uuid[] IS NULL OR NOT (id = ANY($3)))
              AND ($4::text[] IS NULL OR tags && $4)
              AND ($5::text[] IS NULL OR NOT (tags && $5))
              AND ($6::text IS NULL OR description ILIKE '%' || $6 || '%')
              AND ($7::date IS NULL OR datetime::date >= $7)
              AND ($8::date IS NULL OR datetime::date <= $8)
              AND ($9::bigint IS NULL OR EXTRACT(EPOCH FROM duration)::bigint >= $9)
              AND ($10::bigint IS NULL OR EXTRACT(EPOCH FROM duration)::bigint <= $10)
            ORDER BY datetime DESC
            LIMIT $11
            OFFSET $12
            "#,
        )
        .bind(binds.user_id)
        .bind(binds.ids_in.as_deref())
        .bind(binds.ids_not_in.as_deref())
        .bind(binds.tags_in.as_deref())
        .bind(binds.tags_not_in.as_deref())
        .bind(binds.description_contains.as_deref())
        .bind(binds.date_from)
        .bind(binds.date_to)
        .bind(binds.duration_from_secs)
        .bind(binds.duration_to_secs)
        .bind(binds.limit)
        .bind(binds.offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| RepositoryError::QueryFailed)?;

        rows.into_iter().map(row_to_worklog).collect()
    }

    async fn delete(&self, user_id: UserId, id: WorklogId) -> RepositoryResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE worklogs
            SET deleted_at = NOW()
            WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(id.as_uuid())
        .bind(user_id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}
