use chrono::{DateTime, Utc};

use crate::error::{DomainError, DomainResult};
use crate::value_objects::{
    CreatedAt, DeletedAt, DisabledAt, Email, RevokedAt, TokenId, UpdatedAt, UserId, UserName,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
    name: UserName,
    email: Option<Email>,
    password_hash: Option<String>,
    created_at: CreatedAt,
    updated_at: UpdatedAt,
    disabled_at: Option<DisabledAt>,
    deleted_at: Option<DeletedAt>,
}

impl User {
    pub fn create(name: UserName, email: Option<Email>) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::generate(),
            name,
            email,
            password_hash: None,
            created_at: CreatedAt::new(now),
            updated_at: UpdatedAt::new(now),
            disabled_at: None,
            deleted_at: None,
        }
    }

    pub fn reconstitute(
        id: UserId,
        name: UserName,
        email: Option<Email>,
        password_hash: Option<String>,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
        disabled_at: Option<DisabledAt>,
        deleted_at: Option<DeletedAt>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            password_hash,
            created_at,
            updated_at,
            disabled_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }

    pub fn password_hash(&self) -> Option<&str> {
        self.password_hash.as_deref()
    }

    pub fn created_at(&self) -> CreatedAt {
        self.created_at
    }

    pub fn updated_at(&self) -> UpdatedAt {
        self.updated_at
    }

    pub fn disabled_at(&self) -> Option<DisabledAt> {
        self.disabled_at
    }

    pub fn deleted_at(&self) -> Option<DeletedAt> {
        self.deleted_at
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled_at.is_some()
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn is_active(&self) -> bool {
        !self.is_disabled() && !self.is_deleted()
    }

    pub fn disable(&mut self) -> DomainResult<()> {
        if self.is_deleted() {
            return Err(DomainError::UserDeleted);
        }
        if self.is_disabled() {
            return Err(DomainError::AlreadyDisabled);
        }
        self.disabled_at = Some(DisabledAt::new(Utc::now()));
        self.touch();
        Ok(())
    }

    pub fn enable(&mut self) -> DomainResult<()> {
        if self.is_deleted() {
            return Err(DomainError::UserDeleted);
        }
        if !self.is_disabled() {
            return Err(DomainError::NotDisabled);
        }
        self.disabled_at = None;
        self.touch();
        Ok(())
    }

    pub fn soft_delete(&mut self) -> DomainResult<()> {
        if self.is_deleted() {
            return Err(DomainError::AlreadyDeleted);
        }
        self.deleted_at = Some(DeletedAt::new(Utc::now()));
        self.touch();
        Ok(())
    }

    pub fn set_password(&mut self, password_hash: String) -> DomainResult<()> {
        if self.is_deleted() {
            return Err(DomainError::UserDeleted);
        }
        self.password_hash = Some(password_hash);
        self.touch();
        Ok(())
    }

    fn touch(&mut self) {
        self.updated_at = UpdatedAt::new(Utc::now());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiToken {
    id: TokenId,
    user_id: UserId,
    token_hash: Vec<u8>,
    label: String,
    created_at: CreatedAt,
    expires_at: Option<DateTime<Utc>>,
    revoked_at: Option<RevokedAt>,
}

impl ApiToken {
    pub fn create(user_id: UserId, token_hash: Vec<u8>, label: impl Into<String>) -> Self {
        Self {
            id: TokenId::generate(),
            user_id,
            token_hash,
            label: label.into(),
            created_at: CreatedAt::new(Utc::now()),
            expires_at: None,
            revoked_at: None,
        }
    }

    pub fn reconstitute(
        id: TokenId,
        user_id: UserId,
        token_hash: Vec<u8>,
        label: String,
        created_at: CreatedAt,
        expires_at: Option<DateTime<Utc>>,
        revoked_at: Option<RevokedAt>,
    ) -> Self {
        Self {
            id,
            user_id,
            token_hash,
            label,
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

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn created_at(&self) -> CreatedAt {
        self.created_at
    }

    pub fn expires_at(&self) -> Option<DateTime<Utc>> {
        self.expires_at
    }

    pub fn revoked_at(&self) -> Option<RevokedAt> {
        self.revoked_at
    }

    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at
            .is_some_and(|expires| Utc::now() >= expires)
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

#[cfg(test)]
mod tests {
    use super::User;
    use crate::error::DomainError;
    use crate::value_objects::{Email, UserName};

    #[test]
    fn user_disable_and_enable() {
        let mut user = User::create(
            UserName::try_new("Alice").unwrap(),
            Some(Email::try_new("alice@example.com").unwrap()),
        );

        assert!(user.is_active());
        user.disable().unwrap();
        assert!(user.is_disabled());
        assert!(!user.is_active());

        user.enable().unwrap();
        assert!(user.is_active());
    }

    #[test]
    fn user_soft_delete() {
        let mut user = User::create(UserName::try_new("Bob").unwrap(), None);

        user.soft_delete().unwrap();
        assert!(user.is_deleted());
        assert!(!user.is_active());
        assert_eq!(user.disable(), Err(DomainError::UserDeleted));
    }

    #[test]
    fn user_disable_is_idempotent_error() {
        let mut user = User::create(UserName::try_new("Carol").unwrap(), None);
        user.disable().unwrap();
        assert_eq!(user.disable(), Err(DomainError::AlreadyDisabled));
    }

    #[test]
    fn user_soft_delete_is_idempotent_error() {
        let mut user = User::create(UserName::try_new("Dan").unwrap(), None);
        user.soft_delete().unwrap();
        assert_eq!(user.soft_delete(), Err(DomainError::AlreadyDeleted));
    }
}
