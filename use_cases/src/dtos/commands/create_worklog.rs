use serde::Deserialize;

use crate::error::{UseCaseResult, ValidationError};
use crate::jalali::parse_jalali_date;
use domain::value_objects::{DESCRIPTION_MAX_LEN, MAX_TAG_COUNT, TAG_MAX_LEN};

/// Input for the create-worklog use case.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateWorklogCommand {
    /// Jalali calendar date (`YYYY-MM-DD` or `YYYY/MM/DD`). When `None` or blank, today (Asia/Tehran) is used.
    pub jalali_date: Option<String>,
    /// Work session length in seconds (must be > 0 and < 86_400).
    pub duration_secs: u64,
    pub tags: Vec<String>,
    pub description: String,
}

impl CreateWorklogCommand {
    pub fn validate(&self) -> UseCaseResult<()> {
        if let Some(date) = &self.jalali_date {
            if !date.trim().is_empty() {
                parse_jalali_date(date)?;
            }
        }

        if self.duration_secs == 0 {
            return Err(ValidationError::DurationRequired.into());
        }
        if self.duration_secs >= 86_400 {
            return Err(ValidationError::DurationTooLong.into());
        }

        if self.tags.is_empty() {
            return Err(ValidationError::TagsRequired.into());
        }
        if self.tags.len() > MAX_TAG_COUNT {
            return Err(ValidationError::TooManyTags {
                max: MAX_TAG_COUNT,
                count: self.tags.len(),
            }
            .into());
        }
        for tag in &self.tags {
            if tag.trim().is_empty() {
                return Err(ValidationError::EmptyTag.into());
            }
            let len = tag.chars().count();
            if len > TAG_MAX_LEN {
                return Err(domain::error::DomainError::TagTooLong {
                    max: TAG_MAX_LEN,
                    len,
                }
                .into());
            }
        }

        if self.description.trim().is_empty() {
            return Err(ValidationError::DescriptionRequired.into());
        }
        let desc_len = self.description.chars().count();
        if desc_len > DESCRIPTION_MAX_LEN {
            return Err(domain::error::DomainError::DescriptionTooLong {
                max: DESCRIPTION_MAX_LEN,
                len: desc_len,
            }
            .into());
        }

        Ok(())
    }
}
