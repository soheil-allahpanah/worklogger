use url::Url;

use crate::client::WorkloggerClient;
use crate::error::{SdkError, SdkResult};

#[derive(Debug, Default)]
pub struct WorkloggerClientBuilder {
    base_url: Option<String>,
    token: Option<String>,
}

impl WorkloggerClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn build(self) -> SdkResult<WorkloggerClient> {
        let base_url = self
            .base_url
            .ok_or_else(|| SdkError::Config("base_url is required".into()))?;
        let token = self
            .token
            .ok_or_else(|| SdkError::Config("token is required".into()))?;

        let base_url = Url::parse(&base_url)
            .map_err(|e| SdkError::Config(format!("invalid base_url: {e}")))?;

        Ok(WorkloggerClient::new(base_url, token))
    }
}
