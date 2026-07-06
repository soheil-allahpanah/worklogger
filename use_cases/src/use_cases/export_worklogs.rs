use chrono::Utc;
use common::pagination::PagingParams;
use domain::traits::WorklogRepository;

use crate::dtos::commands::FilterWorklogsCommand;
use crate::dtos::responses::ExportWorklogsResponse;
use crate::error::UseCaseResult;
use crate::export::worklogs_to_xlsx;
use crate::mappers::command_to_filter_criteria;

const EXPORT_MAX_ROWS: u32 = 100_000;
const XLSX_CONTENT_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

pub struct ExportWorklogsUsecase<R: WorklogRepository> {
    repository: R,
}

impl<R: WorklogRepository> ExportWorklogsUsecase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        command: FilterWorklogsCommand,
    ) -> UseCaseResult<ExportWorklogsResponse> {
        let mut criteria = command_to_filter_criteria(command)?;
        criteria.paging = PagingParams {
            page: 1,
            size: EXPORT_MAX_ROWS,
        };

        let result = self.repository.filter(&criteria).await?;
        let items = result.items;
        let row_count = items.len();
        let bytes = worklogs_to_xlsx(&items)?;
        let filename = export_filename();

        Ok(ExportWorklogsResponse::new(
            bytes,
            filename,
            XLSX_CONTENT_TYPE,
            row_count,
        ))
    }
}

fn export_filename() -> String {
    let now = Utc::now().format("%Y%m%d_%H%M%S");
    format!("worklogs_{now}.xlsx")
}
