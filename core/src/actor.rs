use crate::value_objects::UserId;

/// Identity of the caller for authorization-scoped operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorContext {
    user_id: UserId,
}

impl ActorContext {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }
}
