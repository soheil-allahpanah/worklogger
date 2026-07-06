use domain::entities::Worklog;
use domain::error::DomainError;

use crate::dtos::commands::EditWorklogCommand;
use crate::error::UseCaseResult;
use crate::mappers::worklog_fields::parse_worklog_fields;

pub fn apply_edit_command(worklog: &mut Worklog, command: EditWorklogCommand) -> UseCaseResult<()> {
    if worklog.is_deleted() {
        return Err(DomainError::AlreadyDeleted.into());
    }

    let fields = parse_worklog_fields(
        command.jalali_date,
        command.duration_secs,
        command.tags,
        command.description,
    )?;

    worklog.set_datetime(fields.datetime);
    worklog.set_duration(fields.duration);
    worklog.set_tags(fields.tags);
    worklog.set_description(Some(fields.description));

    Ok(())
}
