use common::pagination::PageResult;
use domain::traits::WorklogRepository;

use crate::dtos::commands::FilterWorklogsCommand;
use crate::dtos::responses::FilterWorklogsResponse;
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
        let items = self.repository.filter(&criteria).await?;
        let total_items = items.len() as u64;
        Ok(PageResult::new(
            items,
            total_items,
            criteria.paging.page,
            criteria.paging.size,
        ))
    }
}
