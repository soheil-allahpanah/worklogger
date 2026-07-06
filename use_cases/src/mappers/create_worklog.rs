use domain::entities::Worklog;

use crate::dtos::commands::CreateWorklogCommand;
use crate::error::UseCaseResult;
use crate::mappers::worklog_fields::parse_worklog_fields;

pub fn command_to_worklog(command: CreateWorklogCommand) -> UseCaseResult<Worklog> {
    let fields = parse_worklog_fields(
        command.jalali_date,
        command.duration_secs,
        command.tags,
        command.description,
    )?;

    Ok(Worklog::create(
        command.user_id,
        fields.datetime,
        fields.duration,
        fields.tags,
        fields.description,
    ))
}
