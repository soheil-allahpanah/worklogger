use std::sync::Arc;

use crate::criteria::WorklogFilterCriteria;
use crate::entities::Worklog;
use crate::results::{WorklogFilterResult, WorklogTagStatsResult};
use crate::traits::repository_error::RepositoryResult;
use crate::value_objects::{UserId, WorklogId};

/// Persistence port for the `Worklog` aggregate. Implementations live in `infrastructure`.
pub trait WorklogRepository {
    async fn get(&self, user_id: UserId, id: WorklogId) -> RepositoryResult<Worklog>;
    async fn save(&self, worklog: &Worklog) -> RepositoryResult<()>;
    async fn update(&self, worklog: &Worklog) -> RepositoryResult<()>;
    async fn filter(&self, criteria: &WorklogFilterCriteria) -> RepositoryResult<WorklogFilterResult>;
    /// Per-tag duration/days over the full filtered set (ignores paging).
    async fn tag_stats(
        &self,
        criteria: &WorklogFilterCriteria,
    ) -> RepositoryResult<WorklogTagStatsResult>;
    async fn delete(&self, user_id: UserId, id: WorklogId) -> RepositoryResult<()>;
}

impl<R: WorklogRepository> WorklogRepository for Arc<R> {

    async fn get(&self, user_id: UserId, id: WorklogId) -> RepositoryResult<Worklog> {
        self.as_ref().get(user_id, id).await
    }

    async fn save(&self, worklog: &Worklog) -> RepositoryResult<()> {
        self.as_ref().save(worklog).await
    }

    async fn update(&self, worklog: &Worklog) -> RepositoryResult<()> {
        self.as_ref().update(worklog).await
    }

    async fn filter(&self, criteria: &WorklogFilterCriteria) -> RepositoryResult<WorklogFilterResult> {
        self.as_ref().filter(criteria).await
    }

    async fn tag_stats(
        &self,
        criteria: &WorklogFilterCriteria,
    ) -> RepositoryResult<WorklogTagStatsResult> {
        self.as_ref().tag_stats(criteria).await
    }

    async fn delete(&self, user_id: UserId, id: WorklogId) -> RepositoryResult<()> {
        self.as_ref().delete(user_id, id).await
    }
}
