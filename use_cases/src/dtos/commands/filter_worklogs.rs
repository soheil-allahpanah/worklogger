use common::filter::{JalaliDateFilter, ListFilter, TextFilter, DurationFilter};
use common::pagination::PagingParams;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct FilterWorklogsCommand {
    pub tags: Option<ListFilter<String>>,
    pub ids: Option<ListFilter<Uuid>>,
    pub description: Option<TextFilter>,
    pub date: Option<JalaliDateFilter>,
    pub duration: Option<DurationFilter>,
    pub paging: PagingParams,
}

impl FilterWorklogsCommand {
    pub fn validate(&mut self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Delegate to DateFilter validation
        if let Some(date_filter) = &self.date {
            if let Err(mut date_errors) = date_filter.validate() {
                errors.append(&mut date_errors);
            }
        }

        // Delegate to DurationFilter validation
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