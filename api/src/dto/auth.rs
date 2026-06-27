use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthTokensJson {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: &'static str,
}

impl AuthTokensJson {
    pub fn from_response(response: use_cases::AuthTokensResponse) -> Self {
        Self {
            access_token: response.access_token().to_owned(),
            refresh_token: response.refresh_token().to_owned(),
            expires_in: response.expires_in(),
            token_type: "Bearer",
        }
    }
}
