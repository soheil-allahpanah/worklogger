#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeResponse {
    id: String,
    name: String,
    email: Option<String>,
}

impl MeResponse {
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: Option<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}
