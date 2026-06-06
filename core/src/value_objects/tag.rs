use std::fmt::{self, Display, Formatter};
use crate::error::{DomainError, DomainResult};

/// Maximum length of a single tag label (Unicode scalar count).
pub const TAG_MAX_LEN: usize = 30;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

impl Tag {
    pub fn try_new(value: impl Into<String>) -> DomainResult<Self> {
        let value = value.into();
        validate(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<String> for Tag {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl TryFrom<&str> for Tag {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

fn validate(value: &str) -> DomainResult<()> {
    if value.is_empty() {
        return Err(DomainError::EmptyTag);
    }
    let len = value.chars().count();
    if len > TAG_MAX_LEN {
        return Err(DomainError::TagTooLong {
            max: TAG_MAX_LEN,
            len,
        });
    }
    Ok(())
}
