use domain::entities::Worklog;
use domain::value_objects::{Description, Tags, WorklogDuration};

use crate::dtos::commands::CreateWorklogCommand;
use crate::error::UseCaseResult;
use crate::jalali::{jalali_date_to_worklog_datetime, parse_jalali_date, today_jalali_in_tehran};

pub fn command_to_worklog(command: CreateWorklogCommand) -> UseCaseResult<Worklog> {
    let (jy, jm, jd) = match command
        .jalali_date
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(raw) => parse_jalali_date(raw)?,
        None => today_jalali_in_tehran(),
    };

    let datetime = jalali_date_to_worklog_datetime(jy, jm, jd)?;
    let duration = WorklogDuration::try_from_secs(command.duration_secs)?;
    let tags = Tags::try_from_strs(command.tags)?;
    let description = Description::try_new(command.description)?;

    Ok(Worklog::create(
        command.user_id,
        datetime,
        duration,
        tags,
        description,
    ))
}
