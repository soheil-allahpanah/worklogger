use domain::entities::Worklog;
use use_cases::export::worklog_display::{format_description, format_duration_secs, jalali_date_string};

use crate::dto::WorklogJson;

pub fn worklog_to_json(worklog: Worklog) -> WorklogJson {
    let duration_secs = worklog.duration().as_std().as_secs();
    WorklogJson {
        id: worklog.id().to_string(),
        datetime: worklog.datetime().as_datetime(),
        jalali_date: jalali_date_string(worklog.datetime().as_datetime()),
        duration_secs,
        duration: format_duration_secs(duration_secs),
        tags: worklog
            .tags()
            .iter()
            .map(|tag| tag.as_str().to_string())
            .collect(),
        description: format_description(&worklog),
        created_at: worklog.created_at().as_datetime(),
        updated_at: worklog.updated_at().as_datetime(),
        deleted_at: worklog.deleted_at().map(|ts| ts.as_datetime()),
    }
}
