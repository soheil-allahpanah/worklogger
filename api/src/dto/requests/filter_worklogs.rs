use common::filter::{DurationFilter, JalaliDateFilter, ListFilter, TextFilter};
use common::pagination::PagingParams;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct FilterWorklogsRequest {
    pub tags: Option<ListFilter<String>>,
    pub ids: Option<ListFilter<Uuid>>,
    pub description: Option<TextFilter>,
    pub date: Option<JalaliDateFilter>,
    pub duration: Option<DurationFilter>,
    #[serde(default)]
    pub paging: PagingParams,
}

impl FilterWorklogsRequest {
    pub fn validate(&mut self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Some(date_filter) = &self.date {
            if let Err(mut date_errors) = date_filter.validate() {
                errors.append(&mut date_errors);
            }
        }

        if let Some(duration_filter) = &self.duration {
            if let Err(mut duration_errors) = duration_filter.validate() {
                errors.append(&mut duration_errors);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
