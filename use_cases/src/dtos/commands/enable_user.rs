use domain::value_objects::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnableUserCommand {
    pub user_id: UserId,
}
