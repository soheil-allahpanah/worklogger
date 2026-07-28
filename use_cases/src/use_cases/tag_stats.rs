use domain::traits::WorklogRepository;

use crate::dtos::commands::FilterWorklogsCommand;
use crate::dtos::responses::{TagStatResponse, TagStatsResponse};
use crate::error::UseCaseResult;
use crate::mappers::command_to_filter_criteria;

pub struct TagStatsUseCase<R: WorklogRepository> {
    repository: R,
}

impl<R: WorklogRepository> TagStatsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Aggregates per-tag duration and days over the full filtered set.
    /// Paging on the command is ignored.
    pub async fn execute(&self, command: FilterWorklogsCommand) -> UseCaseResult<TagStatsResponse> {
        let criteria = command_to_filter_criteria(command)?;
        let result = self.repository.tag_stats(&criteria).await?;
        Ok(TagStatsResponse {
            tags: result
                .tags
                .into_iter()
                .map(|stat| TagStatResponse {
                    tag: stat.tag,
                    duration_secs: stat.duration_secs,
                    days_worked: stat.days_worked,
                    worklog_count: stat.worklog_count,
                })
                .collect(),
        })
    }
}
