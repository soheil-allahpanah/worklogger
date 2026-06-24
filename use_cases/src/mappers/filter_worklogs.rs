use common::filter::{DateFilter, DurationFilter, ListFilter};
use domain::criteria::{WorklogDurationFilter, WorklogFilterCriteria};
use domain::value_objects::{Tag, WorklogDuration, WorklogId};
use uuid::Uuid;

use crate::dtos::commands::FilterWorklogsCommand;
use crate::error::{UseCaseError, UseCaseResult};

pub fn command_to_filter_criteria(command: FilterWorklogsCommand) -> UseCaseResult<WorklogFilterCriteria> {
    let tags = command
        .tags
        .map(map_string_list_filter)
        .transpose()?;
    let ids = command.ids.map(map_uuid_list_filter);
    let date = command
        .date
        .map(DateFilter::try_from)
        .transpose()
        .map_err(|msg| UseCaseError::Validation(crate::error::ValidationError::InvalidJalaliDate(msg)))?;
    let duration = command.duration.map(map_duration_filter).transpose()?;

    Ok(WorklogFilterCriteria {
        user_id: command.user_id,
        tags,
        ids,
        description: command.description,
        date,
        duration,
        paging: command.paging,
    })
}

fn map_string_list_filter(filter: ListFilter<String>) -> UseCaseResult<ListFilter<Tag>> {
    let in_list = map_tag_list(filter.in_list)?;
    let not_in = map_tag_list(filter.not_in)?;
    Ok(ListFilter::new(in_list, not_in))
}

fn map_tag_list(tags: Option<Vec<String>>) -> UseCaseResult<Option<Vec<Tag>>> {
    match tags {
        None => Ok(None),
        Some(values) => {
            let tags = values
                .into_iter()
                .map(|s| Tag::try_from(s).map_err(UseCaseError::from))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Some(tags))
        }
    }
}

fn map_uuid_list_filter(filter: ListFilter<Uuid>) -> ListFilter<WorklogId> {
    ListFilter::new (
         filter.in_list.map(|uuids| uuids.into_iter().map(WorklogId::from_uuid).collect()),
         filter.not_in.map(|uuids| uuids.into_iter().map(WorklogId::from_uuid).collect()),
    )
}

fn map_duration_filter(filter: DurationFilter) -> UseCaseResult<WorklogDurationFilter> {
    let from = filter
        .from_secs()
        .map(WorklogDuration::try_from_secs)
        .transpose()
        .map_err(UseCaseError::from)?;
    let to = filter
        .to_secs()
        .map(WorklogDuration::try_from_secs)
        .transpose()
        .map_err(UseCaseError::from)?;
    Ok(WorklogDurationFilter { from, to })
}
