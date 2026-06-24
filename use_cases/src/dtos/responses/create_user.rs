use domain::value_objects::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUserResponse {
    id: UserId,
    name: String,
}

impl CreateUserResponse {
    pub fn new(id: UserId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
