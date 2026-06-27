use std::sync::Arc;

use infrastructure::postgres::{
    PostgresRefreshTokenRepository, PostgresTokenRepository, PostgresUserRepository,
    PostgresWorklogRepository,
};
use use_cases::{
    AuthenticateJwtUseCase, AuthenticateTokenUseCase, CreateTokenUseCase, CreateUserUseCase,
    CreateWorklogUseCase, DeleteWorklogUseCase, ExportWorklogsUsecase, FilterWorklogsUsecase,
    GetWorklogUseCase, JwtConfig, LoginUseCase, RefreshAccessTokenUseCase,
    RevokeRefreshTokenUseCase,
};

type WorklogRepo = Arc<PostgresWorklogRepository>;
type UserRepo = Arc<PostgresUserRepository>;
type TokenRepo = Arc<PostgresTokenRepository>;
type RefreshTokenRepo = Arc<PostgresRefreshTokenRepository>;

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
    authenticate_jwt: AuthenticateJwtUseCase<UserRepo>,
    login: LoginUseCase<RefreshTokenRepo, UserRepo>,
    refresh_access_token: RefreshAccessTokenUseCase<RefreshTokenRepo, UserRepo>,
    revoke_refresh_token: RevokeRefreshTokenUseCase<RefreshTokenRepo>,
    create_user: CreateUserUseCase<UserRepo>,
    create_token: CreateTokenUseCase<TokenRepo, UserRepo>,
}

impl AppState {
    pub fn new(
        worklog_repo: WorklogRepo,
        user_repo: UserRepo,
        token_repo: TokenRepo,
        refresh_token_repo: RefreshTokenRepo,
        jwt_config: JwtConfig,
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
                authenticate_jwt: AuthenticateJwtUseCase::new(
                    Arc::clone(&user_repo),
                    jwt_config.clone(),
                ),
                login: LoginUseCase::new(
                    Arc::clone(&refresh_token_repo),
                    Arc::clone(&user_repo),
                    jwt_config.clone(),
                ),
                refresh_access_token: RefreshAccessTokenUseCase::new(
                    Arc::clone(&refresh_token_repo),
                    Arc::clone(&user_repo),
                    jwt_config.clone(),
                ),
                revoke_refresh_token: RevokeRefreshTokenUseCase::new(refresh_token_repo),
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

    pub fn authenticate_jwt(&self) -> &AuthenticateJwtUseCase<UserRepo> {
        &self.inner.authenticate_jwt
    }

    pub fn login(&self) -> &LoginUseCase<RefreshTokenRepo, UserRepo> {
        &self.inner.login
    }

    pub fn refresh_access_token(&self) -> &RefreshAccessTokenUseCase<RefreshTokenRepo, UserRepo> {
        &self.inner.refresh_access_token
    }

    pub fn revoke_refresh_token(&self) -> &RevokeRefreshTokenUseCase<RefreshTokenRepo> {
        &self.inner.revoke_refresh_token
    }

    pub fn create_user(&self) -> &CreateUserUseCase<UserRepo> {
        &self.inner.create_user
    }

    pub fn create_token(&self) -> &CreateTokenUseCase<TokenRepo, UserRepo> {
        &self.inner.create_token
    }
}
