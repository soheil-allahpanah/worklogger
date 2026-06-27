/// Input for the create-user use case (admin-provisioned).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUserCommand {
    pub name: String,
    pub email: Option<String>,
    pub password: Option<String>,
}
