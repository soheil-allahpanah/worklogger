use chrono::NaiveDate;
use common::filter::ListFilter;
use domain::criteria::WorklogFilterCriteria;
use domain::value_objects::{Tag, WorklogId};
use uuid::Uuid;

pub struct FilterBinds {
    pub ids_in: Option<Vec<Uuid>>,
    pub ids_not_in: Option<Vec<Uuid>>,
    pub tags_in: Option<Vec<String>>,
    pub tags_not_in: Option<Vec<String>>,
    pub description_contains: Option<String>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub duration_from_secs: Option<i64>,
    pub duration_to_secs: Option<i64>,
    pub user_id: Uuid,
    pub limit: i64,
    pub offset: i64,
}

impl From<&WorklogFilterCriteria> for FilterBinds {
    fn from(criteria: &WorklogFilterCriteria) -> Self {
        let (ids_in, ids_not_in) = list_filter_ids(criteria.ids.as_ref());
        let (tags_in, tags_not_in) = list_filter_tags(criteria.tags.as_ref());
        let description_contains = criteria
            .description
            .as_ref()
            .and_then(|f| f.contains.clone());
        let (date_from, date_to) = criteria
            .date
            .as_ref()
            .map(|f| (f.from, f.to))
            .unwrap_or((None, None));
        let (duration_from_secs, duration_to_secs) = criteria
            .duration
            .as_ref()
            .map(|f| {
                (
                    f.from.as_ref().map(super::mapper::duration_upper_bound_secs),
                    f.to.as_ref().map(super::mapper::duration_upper_bound_secs),
                )
            })
            .unwrap_or((None, None));

        Self {
            user_id: criteria.user_id.as_uuid(),
            ids_in,
            ids_not_in,
            tags_in,
            tags_not_in,
            description_contains,
            date_from,
            date_to,
            duration_from_secs,
            duration_to_secs,
            limit: criteria.paging.size as i64,
            offset: criteria.paging.offset() as i64,
        }
    }
}

fn list_filter_ids(filter: Option<&ListFilter<WorklogId>>) -> (Option<Vec<Uuid>>, Option<Vec<Uuid>>) {
    match filter {
        None => (None, None),
        Some(f) => (
            f.in_list
                .as_ref()
                .map(|ids| ids.iter().map(|id| id.as_uuid()).collect()),
            f.not_in
                .as_ref()
                .map(|ids| ids.iter().map(|id| id.as_uuid()).collect()),
        ),
    }
}

fn list_filter_tags(filter: Option<&ListFilter<Tag>>) -> (Option<Vec<String>>, Option<Vec<String>>) {
    match filter {
        None => (None, None),
        Some(f) => (
            f.in_list.as_ref().map(|tags| {
                tags.iter()
                    .map(|tag| tag.as_str().to_owned())
                    .collect()
            }),
            f.not_in.as_ref().map(|tags| {
                tags.iter()
                    .map(|tag| tag.as_str().to_owned())
                    .collect()
            }),
        ),
    }
}
