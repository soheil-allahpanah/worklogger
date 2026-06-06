use domain::criteria::WorklogFilterCriteria;
use domain::entities::Worklog;
use domain::value_objects::WorklogId;
use domain::traits::{RepositoryError, RepositoryResult, WorklogRepository};
use sqlx::PgPool;

use super::filter_binds::FilterBinds;
use super::mapper::{duration_secs, row_to_worklog};
use super::row::WorklogRow;


/// PostgreSQL implementation of [`WorklogRepository`] using compile-time-checked SQL (`sqlx` macros).
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

    async fn get(&self, id: WorklogId) -> RepositoryResult<Worklog> {
        let row = sqlx::query_as!(
            WorklogRow,
            r#"
            SELECT id, 
                    datetime,
                    EXTRACT(EPOCH FROM duration)::bigint AS "duration_secs!",
                    tags,
                    description,
                    created_at,
                    updated_at,
                    deleted_at
            FROM worklogs WHERE id = $1
            "#,
            id.as_uuid(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| RepositoryError::QueryFailed)?;

        Ok(row_to_worklog(row)?)
    }

    async fn save(&self, worklog: &Worklog) -> RepositoryResult<()> {
        let duration = duration_secs(worklog.duration())?;
        let tags: Vec<String> = worklog
            .tags()
            .iter()
            .map(|tag| tag.as_str().to_owned())
            .collect();
        let description = worklog.description().map(|d| d.as_str().to_owned());

        sqlx::query!(
            r#"
            INSERT INTO worklogs (
                id,
                datetime,
                duration,
                tags,
                description,
                created_at,
                updated_at,
                deleted_at
            )
            VALUES ($1, $2, make_interval(secs => $3), $4, $5, $6, $7, $8)
            "#,
            worklog.id().as_uuid(),
            worklog.datetime().as_datetime(),
            duration,
            &tags,
            description,
            worklog.created_at().as_datetime(),
            worklog.updated_at().as_datetime(),
            worklog.deleted_at().map(|d| d.as_datetime()),
        )
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        Ok(())
    }

    async fn filter(&self, criteria: &WorklogFilterCriteria) -> RepositoryResult<Vec<Worklog>> {
        let binds = FilterBinds::from(criteria);

        let rows = sqlx::query_as!(
            WorklogRow,
            r#"
            SELECT
                id,
                datetime,
                EXTRACT(EPOCH FROM duration)::bigint AS "duration_secs!",
                tags,
                description,
                created_at,
                updated_at,
                deleted_at
            FROM worklogs
            WHERE deleted_at IS NULL
              AND ($1::uuid[] IS NULL OR id = ANY($1))
              AND ($2::uuid[] IS NULL OR NOT (id = ANY($2)))
              AND ($3::text[] IS NULL OR tags && $3)
              AND ($4::text[] IS NULL OR NOT (tags && $4))
              AND ($5::text IS NULL OR description ILIKE '%' || $5 || '%')
              AND ($6::date IS NULL OR datetime::date >= $6)
              AND ($7::date IS NULL OR datetime::date <= $7)
              AND ($8::bigint IS NULL OR EXTRACT(EPOCH FROM duration)::bigint >= $8)
              AND ($9::bigint IS NULL OR EXTRACT(EPOCH FROM duration)::bigint <= $9)
            ORDER BY datetime DESC
            LIMIT $10
            OFFSET $11
            "#,
            binds.ids_in.as_deref(),
            binds.ids_not_in.as_deref(),
            binds.tags_in.as_deref(),
            binds.tags_not_in.as_deref(),
            binds.description_contains.as_deref(),
            binds.date_from,
            binds.date_to,
            binds.duration_from_secs,
            binds.duration_to_secs,
            binds.limit,
            binds.offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| RepositoryError::QueryFailed)?;

        rows.into_iter().map(row_to_worklog).collect()
    }

    async fn delete(&self, id: WorklogId) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            UPDATE worklogs SET deleted_at = NOW() WHERE id = $1
            "#,
            id.as_uuid(),
        )
        .execute(&self.pool)
        .await
        .map_err(|_| RepositoryError::PersistFailed)?;

        Ok(())
    }
}
