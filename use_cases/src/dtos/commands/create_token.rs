use domain::value_objects::UserId;

/// Input for minting a device token for a user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTokenCommand {
    pub user_id: UserId,
    pub label: String,
}
