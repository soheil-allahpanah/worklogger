use sha2::{Digest, Sha256};
use uuid::Uuid;

const TOKEN_PREFIX: &str = "wl_";
const REFRESH_TOKEN_PREFIX: &str = "rt_";

/// Generates a new raw device token (shown once to the admin/user).
pub fn generate_raw_token() -> String {
    format!("{TOKEN_PREFIX}{}", Uuid::new_v4().simple())
}

/// Generates a new raw refresh token (shown once to the client).
pub fn generate_raw_refresh_token() -> String {
    format!("{REFRESH_TOKEN_PREFIX}{}", Uuid::new_v4().simple())
}

/// SHA-256 hash stored in the database; never persist the raw token.
pub fn hash_token(raw: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::{generate_raw_token, hash_token};

    #[test]
    fn hash_token_is_deterministic() {
        let raw = "wl_test_token";
        assert_eq!(hash_token(raw), hash_token(raw));
    }

    #[test]
    fn generate_raw_token_has_prefix() {
        let raw = generate_raw_token();
        assert!(raw.starts_with("wl_"));
        assert_ne!(raw, generate_raw_token());
    }
}
