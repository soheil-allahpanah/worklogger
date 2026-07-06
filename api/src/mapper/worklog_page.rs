use use_cases::FilterWorklogsResponse;

use crate::dto::{WorklogFilterStatisticsJson, WorklogPageJson};
use crate::mapper::worklog::worklog_to_json;

pub fn page_to_json(response: FilterWorklogsResponse) -> WorklogPageJson {
    let page = response.page;
    WorklogPageJson {
        items: page.items.into_iter().map(worklog_to_json).collect(),
        total_items: page.total_items,
        total_pages: page.total_pages,
        current_page: page.current_page,
        page_size: page.page_size,
        statistics: WorklogFilterStatisticsJson {
            total_duration_secs: response.statistics.total_duration_secs,
            days_worked: response.statistics.days_worked,
        },
    }
}
