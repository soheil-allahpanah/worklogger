use std::fmt::{self, Display, Formatter};

use crate::error::{DomainError, DomainResult};

pub const EMAIL_MAX_LEN: usize = 254;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn try_new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(DomainError::EmptyEmail);
        }
        if trimmed.len() > EMAIL_MAX_LEN {
            return Err(DomainError::EmailTooLong {
                max: EMAIL_MAX_LEN,
                len: trimmed.len(),
            });
        }
        if !trimmed.contains('@') {
            return Err(DomainError::InvalidEmail);
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
