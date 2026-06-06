use std::fmt::{self, Display, Formatter};

use crate::error::{DomainError, DomainResult};

/// Maximum description length in Unicode scalar values (UTF-8 safe).
pub const DESCRIPTION_MAX_LEN: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description(String);

impl Description {
    pub fn try_new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        validate(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for Description {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl TryFrom<&str> for Description {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

fn validate(value: &str) -> DomainResult<()> {
    if value.is_empty() {
        return Err(DomainError::EmptyDescription);
    }
    let len = value.chars().count();
    if len > DESCRIPTION_MAX_LEN {
        return Err(DomainError::DescriptionTooLong {
            max: DESCRIPTION_MAX_LEN,
            len,
        });
    }
    Ok(())
}
