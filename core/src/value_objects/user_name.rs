use std::fmt::{self, Display, Formatter};

use crate::error::{DomainError, DomainResult};

pub const USER_NAME_MAX_LEN: usize = 120;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserName(String);

impl UserName {
    pub fn try_new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(DomainError::EmptyUserName);
        }
        if trimmed.len() > USER_NAME_MAX_LEN {
            return Err(DomainError::UserNameTooLong {
                max: USER_NAME_MAX_LEN,
                len: trimmed.len(),
            });
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for UserName {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
