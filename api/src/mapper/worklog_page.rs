use common::pagination::PageResult;
use domain::entities::Worklog;

use crate::dto::WorklogPageJson;
use crate::mapper::worklog::worklog_to_json;

pub fn page_to_json(page: PageResult<Worklog>) -> WorklogPageJson {
    WorklogPageJson {
        items: page.items.into_iter().map(worklog_to_json).collect(),
        total_items: page.total_items,
        total_pages: page.total_pages,
        current_page: page.current_page,
        page_size: page.page_size,
    }
}
