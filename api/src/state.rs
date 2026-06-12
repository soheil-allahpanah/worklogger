use std::sync::Arc;

use infrastructure::postgres::PostgresWorklogRepository;
use use_cases::{
    CreateWorklogUseCase, DeleteWorklogUseCase, ExportWorklogsUsecase, FilterWorklogsUsecase,
};

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    create_worklog: CreateWorklogUseCase<Arc<PostgresWorklogRepository>>,
    delete_worklog: DeleteWorklogUseCase<Arc<PostgresWorklogRepository>>,
    filter_worklogs: FilterWorklogsUsecase<Arc<PostgresWorklogRepository>>,
    export_worklogs: ExportWorklogsUsecase<Arc<PostgresWorklogRepository>>,
}

impl AppState {
    pub fn new(repo: Arc<PostgresWorklogRepository>) -> Self {
        Self {
            inner: Arc::new(Inner {
                create_worklog: CreateWorklogUseCase::new(Arc::clone(&repo)),
                delete_worklog: DeleteWorklogUseCase::new(Arc::clone(&repo)),
                filter_worklogs: FilterWorklogsUsecase::new(Arc::clone(&repo)),
                export_worklogs: ExportWorklogsUsecase::new(repo),
            }),
        }
    }

    pub fn create_worklog(&self) -> &CreateWorklogUseCase<Arc<PostgresWorklogRepository>> {
        &self.inner.create_worklog
    }

    pub fn delete_worklog(&self) -> &DeleteWorklogUseCase<Arc<PostgresWorklogRepository>> {
        &self.inner.delete_worklog
    }

    pub fn filter_worklogs(&self) -> &FilterWorklogsUsecase<Arc<PostgresWorklogRepository>> {
        &self.inner.filter_worklogs
    }

    pub fn export_worklogs(&self) -> &ExportWorklogsUsecase<Arc<PostgresWorklogRepository>> {
        &self.inner.export_worklogs
    }
}
