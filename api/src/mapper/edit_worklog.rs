use domain::value_objects::UserId;
use use_cases::EditWorklogCommand;
use uuid::Uuid;

use crate::dto::EditWorklogRequest;

pub fn request_to_command(
    request: EditWorklogRequest,
    user_id: UserId,
    id: Uuid,
) -> EditWorklogCommand {
    EditWorklogCommand {
        user_id,
        id,
        jalali_date: request.jalali_date,
        duration_secs: request.duration_secs,
        tags: request.tags,
        description: request.description,
    }
}
