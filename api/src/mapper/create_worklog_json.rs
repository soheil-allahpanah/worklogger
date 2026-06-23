use domain::value_objects::WorklogId;

use crate::dto::CreateWorklogJson;

pub fn worklog_id_to_json(id: WorklogId) -> CreateWorklogJson {
    CreateWorklogJson {
        id: id.to_string(),
    }
}
