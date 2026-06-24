use domain::value_objects::UserId;
use use_cases::CreateWorklogCommand;

use crate::dto::CreateWorklogRequest;

pub fn request_to_command(request: CreateWorklogRequest, user_id: UserId) -> CreateWorklogCommand {
    CreateWorklogCommand {
        user_id,
        jalali_date: request.jalali_date,
        duration_secs: request.duration_secs,
        tags: request.tags,
        description: request.description,
    }
}
