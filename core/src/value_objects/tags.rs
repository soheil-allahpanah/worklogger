use std::fmt::{self, Display, Formatter};
use std::slice::Iter;

use crate::error::{DomainError, DomainResult};
use crate::value_objects::Tag;

pub const MAX_TAG_COUNT: usize = 50;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tags(Vec<Tag>);

impl Tags {
    pub fn try_new(tags: Vec<Tag>) -> DomainResult<Self> {
        Self::validate_count(&tags)?;
        Ok(Self(tags))
    }

    pub fn new(tags: Vec<Tag>) -> Self {
        Self(tags)
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn try_from_strs(values: impl IntoIterator<Item = impl AsRef<str>>) -> DomainResult<Self> {
        let tags: Vec<Tag> = values
            .into_iter()
            .map(|value| Tag::try_from(value.as_ref()))
            .collect::<DomainResult<Vec<_>>>()?;

        Self::validate_count(&tags)?;
        Ok(Self(tags))
    }

    fn validate_count(tags: &[Tag]) -> DomainResult<()> {
        if tags.is_empty() {
            return Err(DomainError::TagsRequired);
        }
        if tags.len() > MAX_TAG_COUNT {
            return Err(DomainError::TooManyTags {
                max: MAX_TAG_COUNT,
                count: tags.len(),
            });
        }
        Ok(())
    }

    pub fn iter(&self) -> Iter<'_, Tag> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_inner(self) -> Vec<Tag> {
        self.0
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let labels: Vec<_> = self.0.iter().map(Tag::as_str).collect();
        write!(f, "[{}]", labels.join(", "))
    }
}

impl FromIterator<Tag> for Tags {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Tags {
    type Item = Tag;
    type IntoIter = std::vec::IntoIter<Tag>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
