use domain::value_objects::{
    Description, Tags, WorklogDateTime, WorklogDuration,
};

use crate::error::UseCaseResult;
use crate::jalali::{jalali_date_to_worklog_datetime, parse_jalali_date, today_jalali_in_tehran};

pub struct WorklogFieldValues {
    pub datetime: WorklogDateTime,
    pub duration: WorklogDuration,
    pub tags: Tags,
    pub description: Description,
}

pub fn parse_worklog_fields(
    jalali_date: Option<String>,
    duration_secs: u64,
    tags: Vec<String>,
    description: String,
) -> UseCaseResult<WorklogFieldValues> {
    let (jy, jm, jd) = match jalali_date
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(raw) => parse_jalali_date(raw)?,
        None => today_jalali_in_tehran(),
    };

    let datetime = jalali_date_to_worklog_datetime(jy, jm, jd)?;
    let duration = WorklogDuration::try_from_secs(duration_secs)?;
    let tags = Tags::try_from_strs(tags)?;
    let description = Description::try_new(description)?;

    Ok(WorklogFieldValues {
        datetime,
        duration,
        tags,
        description,
    })
}
