use domain::value_objects::UserId;

/// Input for setting a user's password (admin).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetPasswordCommand {
    pub user_id: UserId,
    pub password: String,
}
