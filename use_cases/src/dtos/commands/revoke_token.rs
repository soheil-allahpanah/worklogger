use domain::value_objects::TokenId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeTokenCommand {
    pub token_id: TokenId,
}
