use domain::entities::{ApiToken, User};
use domain::traits::RepositoryError;
use domain::traits::RepositoryResult;
use domain::value_objects::{
    CreatedAt, DeletedAt, DisabledAt, Email, RevokedAt, TokenId, UpdatedAt, UserId, UserName,
};

use super::user_row::{TokenRow, UserRow};

pub fn row_to_user(row: UserRow) -> RepositoryResult<User> {
    let name = UserName::try_new(row.name).map_err(|_| RepositoryError::QueryFailed)?;
    let email = row
        .email
        .map(|value| Email::try_new(value).map_err(|_| RepositoryError::QueryFailed))
        .transpose()?;

    Ok(User::reconstitute(
        UserId::from_uuid(row.id),
        name,
        email,
        row.password_hash,
        CreatedAt::new(row.created_at),
        UpdatedAt::new(row.updated_at),
        row.disabled_at.map(DisabledAt::new),
        row.deleted_at.map(DeletedAt::new),
    ))
}

pub fn row_to_token(row: TokenRow) -> RepositoryResult<ApiToken> {
    Ok(ApiToken::reconstitute(
        TokenId::from_uuid(row.id),
        UserId::from_uuid(row.user_id),
        row.token_hash,
        row.label,
        CreatedAt::new(row.created_at),
        row.expires_at,
        row.revoked_at.map(RevokedAt::new),
    ))
}
