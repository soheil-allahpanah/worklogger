use chrono::{DateTime, Utc};

use crate::error::{DomainError, DomainResult};
use crate::value_objects::{CreatedAt, RevokedAt, TokenId, UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshToken {
    id: TokenId,
    user_id: UserId,
    token_hash: Vec<u8>,
    created_at: CreatedAt,
    expires_at: DateTime<Utc>,
    revoked_at: Option<RevokedAt>,
}

impl RefreshToken {
    pub fn create(user_id: UserId, token_hash: Vec<u8>, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: TokenId::generate(),
            user_id,
            token_hash,
            created_at: CreatedAt::new(Utc::now()),
            expires_at,
            revoked_at: None,
        }
    }

    pub fn reconstitute(
        id: TokenId,
        user_id: UserId,
        token_hash: Vec<u8>,
        created_at: CreatedAt,
        expires_at: DateTime<Utc>,
        revoked_at: Option<RevokedAt>,
    ) -> Self {
        Self {
            id,
            user_id,
            token_hash,
            created_at,
            expires_at,
            revoked_at,
        }
    }

    pub fn id(&self) -> TokenId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn token_hash(&self) -> &[u8] {
        &self.token_hash
    }

    pub fn created_at(&self) -> CreatedAt {
        self.created_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn revoked_at(&self) -> Option<RevokedAt> {
        self.revoked_at
    }

    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        !self.is_revoked() && !self.is_expired()
    }

    pub fn revoke(&mut self) -> DomainResult<()> {
        if self.is_revoked() {
            return Err(DomainError::TokenAlreadyRevoked);
        }
        self.revoked_at = Some(RevokedAt::new(Utc::now()));
        Ok(())
    }
}
