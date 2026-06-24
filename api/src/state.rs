use std::sync::Arc;

use infrastructure::postgres::{
    PostgresTokenRepository, PostgresUserRepository, PostgresWorklogRepository,
};
use use_cases::{
    AuthenticateTokenUseCase, CreateTokenUseCase, CreateUserUseCase, CreateWorklogUseCase,
    DeleteWorklogUseCase, ExportWorklogsUsecase, FilterWorklogsUsecase, GetWorklogUseCase,
};

type WorklogRepo = Arc<PostgresWorklogRepository>;
type UserRepo = Arc<PostgresUserRepository>;
type TokenRepo = Arc<PostgresTokenRepository>;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    create_worklog: CreateWorklogUseCase<WorklogRepo>,
    get_worklog: GetWorklogUseCase<WorklogRepo>,
    delete_worklog: DeleteWorklogUseCase<WorklogRepo>,
    filter_worklogs: FilterWorklogsUsecase<WorklogRepo>,
    export_worklogs: ExportWorklogsUsecase<WorklogRepo>,
    authenticate_token: AuthenticateTokenUseCase<TokenRepo, UserRepo>,
    create_user: CreateUserUseCase<UserRepo>,
    create_token: CreateTokenUseCase<TokenRepo, UserRepo>,
}

impl AppState {
    pub fn new(
        worklog_repo: WorklogRepo,
        user_repo: UserRepo,
        token_repo: TokenRepo,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                create_worklog: CreateWorklogUseCase::new(Arc::clone(&worklog_repo)),
                get_worklog: GetWorklogUseCase::new(Arc::clone(&worklog_repo)),
                delete_worklog: DeleteWorklogUseCase::new(Arc::clone(&worklog_repo)),
                filter_worklogs: FilterWorklogsUsecase::new(Arc::clone(&worklog_repo)),
                export_worklogs: ExportWorklogsUsecase::new(worklog_repo),
                authenticate_token: AuthenticateTokenUseCase::new(
                    Arc::clone(&token_repo),
                    Arc::clone(&user_repo),
                ),
                create_user: CreateUserUseCase::new(Arc::clone(&user_repo)),
                create_token: CreateTokenUseCase::new(token_repo, user_repo),
            }),
        }
    }

    pub fn create_worklog(&self) -> &CreateWorklogUseCase<WorklogRepo> {
        &self.inner.create_worklog
    }

    pub fn get_worklog(&self) -> &GetWorklogUseCase<WorklogRepo> {
        &self.inner.get_worklog
    }

    pub fn delete_worklog(&self) -> &DeleteWorklogUseCase<WorklogRepo> {
        &self.inner.delete_worklog
    }

    pub fn filter_worklogs(&self) -> &FilterWorklogsUsecase<WorklogRepo> {
        &self.inner.filter_worklogs
    }

    pub fn export_worklogs(&self) -> &ExportWorklogsUsecase<WorklogRepo> {
        &self.inner.export_worklogs
    }

    pub fn authenticate_token(&self) -> &AuthenticateTokenUseCase<TokenRepo, UserRepo> {
        &self.inner.authenticate_token
    }

    pub fn create_user(&self) -> &CreateUserUseCase<UserRepo> {
        &self.inner.create_user
    }

    pub fn create_token(&self) -> &CreateTokenUseCase<TokenRepo, UserRepo> {
        &self.inner.create_token
    }
}
