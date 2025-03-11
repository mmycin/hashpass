use hashpass::{hash_password, verify_password};
use bcrypt::DEFAULT_COST;

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