use domain::entities::Worklog;
use domain::traits::RepositoryError;
use domain::traits::RepositoryResult;
use domain::value_objects::{
    CreatedAt, DeletedAt, Description, Tag, Tags, UpdatedAt, UserId, WorklogDateTime,
    WorklogDuration, WorklogId,
};

use super::row::WorklogRow;
use super::row::WorklogFilterRow;

pub fn row_to_worklog(row: WorklogRow) -> RepositoryResult<Worklog> {
    reconstitute_worklog(
        row.id,
        row.user_id,
        row.datetime,
        row.duration_secs,
        row.tags,
        row.description,
        row.created_at,
        row.updated_at,
        row.deleted_at,
    )
}

pub fn filter_row_to_worklog(row: WorklogFilterRow) -> RepositoryResult<Worklog> {
    reconstitute_worklog(
        row.id,
        row.user_id,
        row.datetime,
        row.duration_secs,
        row.tags,
        row.description,
        row.created_at,
        row.updated_at,
        row.deleted_at,
    )
}

fn reconstitute_worklog(
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    datetime: chrono::DateTime<chrono::Utc>,
    duration_secs: i64,
    tags: Vec<String>,
    description: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
) -> RepositoryResult<Worklog> {
    let duration = WorklogDuration::try_from_secs(duration_secs as u64)
        .map_err(|_| RepositoryError::QueryFailed)?;
    let tags: Vec<Tag> = tags
        .into_iter()
        .map(|label| Tag::try_from(label).map_err(|_| RepositoryError::QueryFailed))
        .collect::<Result<Vec<_>, _>>()?;
    let description = description
        .map(|text| Description::try_from(text).map_err(|_| RepositoryError::QueryFailed))
        .transpose()?;

    Ok(Worklog::reconstitute(
        WorklogId::from_uuid(id),
        UserId::from_uuid(user_id),
        WorklogDateTime::new(datetime),
        duration,
        Tags::new(tags),
        description,
        CreatedAt::new(created_at),
        UpdatedAt::new(updated_at),
        deleted_at.map(DeletedAt::new),
    ))
}

pub fn duration_secs(duration: WorklogDuration) -> RepositoryResult<f64> {
    let secs = duration.as_timedelta().num_seconds();
    if secs <= 0 {
        return Err(RepositoryError::PersistFailed);
    }
    Ok(secs as f64)
}

pub fn duration_upper_bound_secs(duration: &WorklogDuration) -> i64 {
    duration.as_timedelta().num_seconds()
}
