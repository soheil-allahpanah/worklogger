/// Input for the login use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginCommand {
    pub login: String,
    pub password: String,
}
