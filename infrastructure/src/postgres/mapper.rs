use domain::entities::Worklog;
use domain::traits::RepositoryError;
use domain::traits::RepositoryResult;
use domain::value_objects::{
    CreatedAt, DeletedAt, Description, Tag, Tags, UpdatedAt, WorklogDateTime, WorklogDuration,
    WorklogId,
};

use super::row::WorklogRow;

pub fn row_to_worklog(row: WorklogRow) -> RepositoryResult<Worklog> {
    let duration = WorklogDuration::try_from_secs(row.duration_secs as u64)
        .map_err(|_| RepositoryError::QueryFailed)?;
    let tags: Vec<Tag> = row
        .tags
        .into_iter()
        .map(|label| Tag::try_from(label).map_err(|_| RepositoryError::QueryFailed))
        .collect::<Result<Vec<_>, _>>()?;
    let description = row
        .description
        .map(|text| Description::try_from(text).map_err(|_| RepositoryError::QueryFailed))
        .transpose()?;

    Ok(Worklog::reconstitute(
        WorklogId::from_uuid(row.id),
        WorklogDateTime::new(row.datetime),
        duration,
        Tags::new(tags),
        description,
        CreatedAt::new(row.created_at),
        UpdatedAt::new(row.updated_at),
        row.deleted_at.map(DeletedAt::new),
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
