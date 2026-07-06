use common::pagination::PageResult;
use domain::traits::WorklogRepository;

use crate::dtos::commands::FilterWorklogsCommand;
use crate::dtos::responses::{FilterWorklogsResponse, WorklogFilterStatistics};
use crate::error::UseCaseResult;
use crate::mappers::command_to_filter_criteria;
    
pub struct FilterWorklogsUsecase<R: WorklogRepository> {
    repository: R,
}

impl<R: WorklogRepository> FilterWorklogsUsecase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: WorklogRepository> FilterWorklogsUsecase<R> {
    pub async fn execute(&self, command: FilterWorklogsCommand) -> UseCaseResult<FilterWorklogsResponse> {
        let criteria = command_to_filter_criteria(command)?;
        let result = self.repository.filter(&criteria).await?;
        Ok(FilterWorklogsResponse {
            page: PageResult::new(
                result.items,
                result.total_items,
                criteria.paging.page,
                criteria.paging.size,
            ),
            statistics: WorklogFilterStatistics {
                total_duration_secs: result.total_duration_secs,
                days_worked: result.days_worked,
            },
        })
    }
}
