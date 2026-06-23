use use_cases::CreateWorklogCommand;

use crate::dto::CreateWorklogRequest;

pub fn request_to_command(request: CreateWorklogRequest) -> CreateWorklogCommand {
    CreateWorklogCommand {
        jalali_date: request.jalali_date,
        duration_secs: request.duration_secs,
        tags: request.tags,
        description: request.description,
    }
}
