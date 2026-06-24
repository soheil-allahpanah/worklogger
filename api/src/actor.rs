use domain::bootstrap::legacy_user_id;
use domain::value_objects::UserId;

/// Temporary stand-in for authenticated user identity until Phase 2 auth wiring.
pub fn actor_user_id() -> UserId {
    legacy_user_id()
}
