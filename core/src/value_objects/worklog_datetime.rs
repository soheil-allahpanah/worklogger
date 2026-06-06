use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, Utc};

/// When the logged work actually occurred (`worklogs.datetime`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorklogDateTime(DateTime<Utc>);

impl WorklogDateTime {
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }

    pub fn as_datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Display for WorklogDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<DateTime<Utc>> for WorklogDateTime {
    fn from(datetime: DateTime<Utc>) -> Self {
        Self::new(datetime)
    }
}
