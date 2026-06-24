use chrono::NaiveDate;
use common::filter::{DateFilter, ListFilter, TextFilter};

use crate::criteria::{WorklogDurationFilter, WorklogFilterCriteria};
use crate::entities::Worklog;
use crate::value_objects::{Tag, WorklogId};

pub fn matches_criteria(worklog: &Worklog, criteria: &WorklogFilterCriteria) -> bool {
    if worklog.is_deleted() {
        return false;
    }

    if worklog.user_id() != criteria.user_id {
        return false;
    }

    if let Some(filter) = &criteria.ids {
        if !matches_id_filter(worklog.id(), filter) {
            return false;
        }
    }

    if let Some(filter) = &criteria.tags {
        if !matches_tag_filter(worklog.tags().iter().map(|t| t.as_str()), filter) {
            return false;
        }
    }

    if let Some(filter) = &criteria.description {
        if !matches_description_filter(worklog.description().map(|d| d.as_str()), filter) {
            return false;
        }
    }

    if let Some(filter) = &criteria.date {
        if !matches_date_filter(worklog.datetime().as_datetime().date_naive(), filter) {
            return false;
        }
    }

    if let Some(filter) = &criteria.duration {
        if !matches_duration_filter(worklog.duration(), filter) {
            return false;
        }
    }

    true
}

fn matches_id_filter(id: WorklogId, filter: &ListFilter<WorklogId>) -> bool {
    if let Some(in_list) = &filter.in_list {
        if !in_list.contains(&id) {
            return false;
        }
    }
    if let Some(not_in) = &filter.not_in {
        if not_in.contains(&id) {
            return false;
        }
    }
    true
}

fn matches_tag_filter<'a, I>(tags: I, filter: &ListFilter<Tag>) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let tag_strs: Vec<&str> = tags.collect();
    if let Some(in_list) = &filter.in_list {
        let required: Vec<&str> = in_list.iter().map(|t| t.as_str()).collect();
        if !required.iter().any(|t| tag_strs.contains(t)) {
            return false;
        }
    }
    if let Some(not_in) = &filter.not_in {
        let excluded: Vec<&str> = not_in.iter().map(|t| t.as_str()).collect();
        if excluded.iter().any(|t| tag_strs.contains(t)) {
            return false;
        }
    }
    true
}

fn matches_description_filter(description: Option<&str>, filter: &TextFilter) -> bool {
    let Some(needle) = filter.contains.as_deref() else {
        return true;
    };
    let Some(haystack) = description else {
        return false;
    };
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

fn matches_date_filter(date: NaiveDate, filter: &DateFilter) -> bool {
    if let Some(from) = filter.from {
        if date < from {
            return false;
        }
    }
    if let Some(to) = filter.to {
        if date > to {
            return false;
        }
    }
    true
}

fn matches_duration_filter(
    duration: crate::value_objects::WorklogDuration,
    filter: &WorklogDurationFilter,
) -> bool {
    let secs = duration.as_std().as_secs() as i64;
    if let Some(from) = filter.from {
        if secs < from.as_std().as_secs() as i64 {
            return false;
        }
    }
    if let Some(to) = filter.to {
        if secs > to.as_std().as_secs() as i64 {
            return false;
        }
    }
    true
}

pub fn apply_paging(mut items: Vec<Worklog>, criteria: &WorklogFilterCriteria) -> Vec<Worklog> {
    items.sort_by(|a, b| {
        b.datetime()
            .as_datetime()
            .cmp(&a.datetime().as_datetime())
    });

    let offset = criteria.paging.offset() as usize;
    let limit = criteria.paging.size as usize;
    items.into_iter().skip(offset).take(limit).collect()
}
