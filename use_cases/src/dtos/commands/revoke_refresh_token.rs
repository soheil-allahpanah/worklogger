/// Input for revoking a refresh token (logout).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeRefreshTokenCommand {
    pub refresh_token: String,
}
