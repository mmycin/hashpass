use bcrypt::{hash, verify, DEFAULT_COST};
use thiserror::Error;

pub mod file_handler;

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