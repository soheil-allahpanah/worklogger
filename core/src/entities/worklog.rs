use chrono::Utc;

use crate::error::{DomainError, DomainResult};
use crate::value_objects::{
    CreatedAt, DeletedAt, Description, Tags, UpdatedAt, WorklogDateTime, WorklogDuration,
    WorklogId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Worklog {
    id: WorklogId,
    datetime: WorklogDateTime,
    duration: WorklogDuration,
    tags: Tags,
    description: Option<Description>,
    created_at: CreatedAt,
    updated_at: UpdatedAt,
    deleted_at: Option<DeletedAt>,
}

impl Worklog {
    /// Creates a new worklog before persistence. IDs and audit timestamps are set in the domain.
    pub fn create(
        datetime: WorklogDateTime,
        duration: WorklogDuration,
        tags: Tags,
        description: Description,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: WorklogId::generate(),
            datetime,
            duration,
            tags,
            description: Some(description),
            created_at: CreatedAt::new(now),
            updated_at: UpdatedAt::new(now),
            deleted_at: None,
        }
    }

    /// Rebuilds a worklog loaded from persistence (repository / mapper).
    pub fn reconstitute(
        id: WorklogId,
        datetime: WorklogDateTime,
        duration: WorklogDuration,
        tags: Tags,
        description: Option<Description>,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
        deleted_at: Option<DeletedAt>,
    ) -> Self {
        Self {
            id,
            datetime,
            duration,
            tags,
            description,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> WorklogId {
        self.id
    }

    pub fn datetime(&self) -> WorklogDateTime {
        self.datetime
    }

    pub fn duration(&self) -> WorklogDuration {
        self.duration
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    pub fn created_at(&self) -> CreatedAt {
        self.created_at
    }

    pub fn updated_at(&self) -> UpdatedAt {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<DeletedAt> {
        self.deleted_at
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn soft_delete(&mut self) -> DomainResult<()> {
        if self.is_deleted() {
            return Err(DomainError::AlreadyDeleted);
        }
        self.deleted_at = Some(DeletedAt::new(Utc::now()));
        self.touch();
        Ok(())
    }

    pub fn restore(&mut self) -> DomainResult<()> {
        let Some(_) = self.deleted_at.take() else {
            return Err(DomainError::NotDeleted);
        };
        self.touch();
        Ok(())
    }

    pub fn set_datetime(&mut self, datetime: WorklogDateTime) {
        self.datetime = datetime;
        self.touch();
    }

    pub fn set_duration(&mut self, duration: WorklogDuration) {
        self.duration = duration;
        self.touch();
    }

    pub fn set_tags(&mut self, tags: Tags) {
        self.tags = tags;
        self.touch();
    }

    pub fn set_description(&mut self, description: Option<Description>) {
        self.description = description;
        self.touch();
    }

    fn touch(&mut self) {
        self.updated_at = UpdatedAt::new(Utc::now());
    }
}
