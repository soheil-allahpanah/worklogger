use std::fmt::{self, Display, Formatter};
use std::time::Duration as StdDuration;

use chrono::TimeDelta;

use crate::error::{DomainError, DomainResult};

/// Upper bound for a single work session (exclusive): must be strictly less than 24 hours.
pub const MAX_WORKLOG_DURATION: TimeDelta = TimeDelta::seconds(86_400);

/// How long the logged work lasted (`worklogs.duration` as PostgreSQL `INTERVAL`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorklogDuration(TimeDelta);

impl WorklogDuration {
    pub fn try_new(duration: TimeDelta) -> DomainResult<Self> {
        validate(duration)?;
        Ok(Self(duration))
    }

    pub fn try_from_std(duration: StdDuration) -> DomainResult<Self> {
        let duration = TimeDelta::from_std(duration)
            .map_err(|_| DomainError::InvalidDuration)?;
        Self::try_new(duration)
    }

    pub fn try_from_secs(secs: u64) -> DomainResult<Self> {
        if secs == 0 {
            return Err(DomainError::InvalidDuration);
        }
        let duration = TimeDelta::seconds(secs as i64);
        Self::try_new(duration)
    }

    pub fn as_timedelta(&self) -> TimeDelta {
        self.0
    }

    pub fn as_std(&self) -> StdDuration {
        self.0.to_std().expect("worklog duration fits in std::time::Duration")
    }
}

impl Display for WorklogDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<WorklogDuration> for TimeDelta {
    fn from(value: WorklogDuration) -> Self {
        value.0
    }
}

impl TryFrom<TimeDelta> for WorklogDuration {
    type Error = DomainError;

    fn try_from(duration: TimeDelta) -> Result<Self, Self::Error> {
        Self::try_new(duration)
    }
}

impl TryFrom<StdDuration> for WorklogDuration {
    type Error = DomainError;

    fn try_from(duration: StdDuration) -> Result<Self, Self::Error> {
        Self::try_from_std(duration)
    }
}

fn validate(duration: TimeDelta) -> DomainResult<()> {
    if duration <= TimeDelta::zero() {
        return Err(DomainError::InvalidDuration);
    }
    if duration >= MAX_WORKLOG_DURATION {
        return Err(DomainError::DurationTooLong);
    }
    Ok(())
}
