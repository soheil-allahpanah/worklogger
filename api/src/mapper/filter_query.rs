use common::filter::{DurationFilter, JalaliDateFilter, TextFilter};
use common::pagination::PagingParams;

use crate::dto::{FilterQuery, FilterWorklogsRequest, DEFAULT_PAGE_SIZE};
use crate::helpers::{list_filter_from_csv, uuid_list_filter_from_csv};

pub fn query_to_request(query: FilterQuery) -> FilterWorklogsRequest {
    let tags = list_filter_from_csv(query.tags, query.exclude_tags);
    let ids = uuid_list_filter_from_csv(query.ids, query.exclude_ids);
    let description = query.description.map(|contains| TextFilter {
        contains: Some(contains),
    });
    let date = match (query.date_from, query.date_to) {
        (None, None) => None,
        (from, to) => Some(JalaliDateFilter { from, to }),
    };
    let duration = match (query.duration_from, query.duration_to) {
        (None, None) => None,
        (from, to) => Some(DurationFilter { from, to }),
    };
    let paging = PagingParams {
        page: if query.page == 0 { 1 } else { query.page },
        size: if query.size == 0 { DEFAULT_PAGE_SIZE } else { query.size },
    };

    FilterWorklogsRequest {
        tags,
        ids,
        description,
        date,
        duration,
        paging,
    }
}
