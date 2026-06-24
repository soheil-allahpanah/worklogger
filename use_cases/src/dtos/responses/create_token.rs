use domain::value_objects::TokenId;

#[derive(Clone, PartialEq, Eq)]
pub struct CreateTokenResponse {
    id: TokenId,
    token: String,
}

impl CreateTokenResponse {
    pub fn new(id: TokenId, token: String) -> Self {
        Self { id, token }
    }

    pub fn id(&self) -> TokenId {
        self.id
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl std::fmt::Debug for CreateTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateTokenResponse")
            .field("id", &self.id)
            .field("token", &"<redacted>")
            .finish()
    }
}
