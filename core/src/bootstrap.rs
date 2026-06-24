use uuid::uuid;

use crate::value_objects::UserId;

/// Fixed ID for the bootstrap user created during migration of existing worklogs.
pub fn legacy_user_id() -> UserId {
    UserId::from_uuid(uuid!("00000000-0000-4000-8000-000000000001"))
}
