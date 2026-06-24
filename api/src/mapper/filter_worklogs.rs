use crate::actor::actor_user_id;
use use_cases::FilterWorklogsCommand;

use crate::dto::FilterWorklogsRequest;

pub fn request_to_command(request: FilterWorklogsRequest) -> FilterWorklogsCommand {
    FilterWorklogsCommand {
        user_id: actor_user_id(),
        tags: request.tags,
        ids: request.ids,
        description: request.description,
        date: request.date,
        duration: request.duration,
        paging: request.paging,
    }
}
