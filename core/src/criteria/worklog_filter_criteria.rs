use common::filter::{DateFilter, ListFilter, TextFilter};
use common::pagination::PagingParams;

use crate::criteria::WorklogDurationFilter;
use crate::value_objects::{Tag, WorklogId};

/// Domain criteria for querying worklogs. Built from the application command after validation.
#[derive(Debug, Clone)]
pub struct WorklogFilterCriteria {
    pub tags: Option<ListFilter<Tag>>,
    pub ids: Option<ListFilter<WorklogId>>,
    pub description: Option<TextFilter>,
    pub date: Option<DateFilter>,
    pub duration: Option<WorklogDurationFilter>,
    pub paging: PagingParams,
}
