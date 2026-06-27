use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use domain::value_objects::UserId;

use crate::error::{AuthError, UseCaseResult};

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_ttl_secs: i64,
    pub refresh_ttl_secs: i64,
}

impl JwtConfig {
    pub fn access_ttl(&self) -> Duration {
        Duration::seconds(self.access_ttl_secs)
    }

    pub fn refresh_ttl(&self) -> Duration {
        Duration::seconds(self.refresh_ttl_secs)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    sub: String,
    exp: i64,
    iat: i64,
}

pub fn issue_access_token(config: &JwtConfig, user_id: UserId) -> UseCaseResult<String> {
    let now = Utc::now();
    let exp = now + config.access_ttl();
    let claims = AccessTokenClaims {
        sub: user_id.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|_| AuthError::InvalidToken.into())
}

pub fn validate_access_token(config: &JwtConfig, token: &str) -> UseCaseResult<UserId> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let token_data = decode::<AccessTokenClaims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )
    .map_err(|_| AuthError::InvalidToken)?;

    token_data
        .claims
        .sub
        .parse::<UserId>()
        .map_err(|_| AuthError::InvalidToken.into())
}
