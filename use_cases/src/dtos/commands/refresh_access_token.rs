/// Input for the refresh-token use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshAccessTokenCommand {
    pub refresh_token: String,
}
