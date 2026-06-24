use domain::value_objects::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteUserCommand {
    pub user_id: UserId,
}
