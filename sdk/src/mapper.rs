use std::str::FromStr;

use domain::entities::Worklog;
use domain::value_objects::{
    CreatedAt, DeletedAt, Description, Tag, Tags, UpdatedAt, UserId, WorklogDateTime,
    WorklogDuration, WorklogId,
};

use crate::api_types::WorklogJson;
use crate::error::{SdkError, SdkResult};

pub fn json_to_worklog(json: WorklogJson) -> SdkResult<Worklog> {
    let id = WorklogId::from_str(&json.id).map_err(|e| SdkError::InvalidResponse(e.to_string()))?;
    let user_id =
        UserId::from_str(&json.user_id).map_err(|e| SdkError::InvalidResponse(e.to_string()))?;
    let duration = WorklogDuration::try_from_secs(json.duration_secs)
        .map_err(|e| SdkError::InvalidResponse(e.to_string()))?;
    let tags: Vec<Tag> = json
        .tags
        .into_iter()
        .map(|label| Tag::try_from(label).map_err(|e| SdkError::InvalidResponse(e.to_string())))
        .collect::<Result<Vec<_>, _>>()?;
    let description = if json.description.is_empty() {
        None
    } else {
        Some(
            Description::try_new(json.description)
                .map_err(|e| SdkError::InvalidResponse(e.to_string()))?,
        )
    };

    Ok(Worklog::reconstitute(
        id,
        user_id,
        WorklogDateTime::new(json.datetime),
        duration,
        Tags::new(tags),
        description,
        CreatedAt::new(json.created_at),
        UpdatedAt::new(json.updated_at),
        json.deleted_at.map(DeletedAt::new),
    ))
}
