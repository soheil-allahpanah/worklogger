use chrono::Utc;
use domain::entities::RefreshToken;
use domain::traits::RefreshTokenRepository;
use domain::value_objects::UserId;

use crate::auth::{generate_raw_refresh_token, hash_token, issue_access_token, JwtConfig};
use crate::dtos::responses::AuthTokensResponse;
use crate::error::UseCaseResult;

pub async fn issue_auth_tokens<R: RefreshTokenRepository>(
    refresh_token_repository: &R,
    jwt_config: &JwtConfig,
    user_id: UserId,
) -> UseCaseResult<AuthTokensResponse> {
    let access_token = issue_access_token(jwt_config, user_id)?;
    let raw_refresh = generate_raw_refresh_token();
    let refresh_hash = hash_token(&raw_refresh);
    let expires_at = Utc::now() + jwt_config.refresh_ttl();
    let refresh_token = RefreshToken::create(user_id, refresh_hash, expires_at);
    refresh_token_repository.save(&refresh_token).await?;

    Ok(AuthTokensResponse::new(
        access_token,
        raw_refresh,
        jwt_config.access_ttl_secs,
    ))
}
