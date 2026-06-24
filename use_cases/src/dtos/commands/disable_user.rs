use domain::value_objects::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisableUserCommand {
    pub user_id: UserId,
}
