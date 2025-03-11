use bcrypt::{hash, verify, DEFAULT_COST};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error("Failed to hash password: {0}")]
    HashingError(#[from] bcrypt::BcryptError),
}

/// Hash a password string with the specified number of rounds
pub fn hash_password(password: &str, rounds: u32) -> Result<String, HashError> {
    Ok(hash(password.as_bytes(), rounds)?)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, HashError> {
    Ok(verify(password.as_bytes(), hash)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "test_password";
        let hash = hash_password(password, DEFAULT_COST).unwrap();
        assert!(verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_wrong_password() {
        let password = "test_password";
        let wrong_password = "wrong_password";
        let hash = hash_password(password, DEFAULT_COST).unwrap();
        assert!(!verify_password(wrong_password, &hash).unwrap());
    }
}