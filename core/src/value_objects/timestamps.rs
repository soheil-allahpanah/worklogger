use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreatedAt(DateTime<Utc>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdatedAt(DateTime<Utc>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeletedAt(DateTime<Utc>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisabledAt(DateTime<Utc>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RevokedAt(DateTime<Utc>);

impl CreatedAt {
    pub fn new(at: DateTime<Utc>) -> Self {
        Self(at)
    }

    pub fn as_datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

impl UpdatedAt {
    pub fn new(at: DateTime<Utc>) -> Self {
        Self(at)
    }

    pub fn as_datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

impl DeletedAt {
    pub fn new(at: DateTime<Utc>) -> Self {
        Self(at)
    }

    pub fn as_datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

impl DisabledAt {
    pub fn new(at: DateTime<Utc>) -> Self {
        Self(at)
    }

    pub fn as_datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

impl RevokedAt {
    pub fn new(at: DateTime<Utc>) -> Self {
        Self(at)
    }

    pub fn as_datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Display for CreatedAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for UpdatedAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for DeletedAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for DisabledAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Display for RevokedAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<DateTime<Utc>> for CreatedAt {
    fn from(at: DateTime<Utc>) -> Self {
        Self::new(at)
    }
}

impl From<DateTime<Utc>> for UpdatedAt {
    fn from(at: DateTime<Utc>) -> Self {
        Self::new(at)
    }
}

impl From<DateTime<Utc>> for DeletedAt {
    fn from(at: DateTime<Utc>) -> Self {
        Self::new(at)
    }
}

impl From<DateTime<Utc>> for DisabledAt {
    fn from(at: DateTime<Utc>) -> Self {
        Self::new(at)
    }
}

impl From<DateTime<Utc>> for RevokedAt {
    fn from(at: DateTime<Utc>) -> Self {
        Self::new(at)
    }
}
