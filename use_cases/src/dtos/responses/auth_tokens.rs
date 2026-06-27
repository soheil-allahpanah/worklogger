/// Token pair returned by login and refresh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthTokensResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

impl AuthTokensResponse {
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        Self {
            access_token,
            refresh_token,
            expires_in,
        }
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn expires_in(&self) -> i64 {
        self.expires_in
    }
}
