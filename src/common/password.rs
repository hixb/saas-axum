use std::fmt::format;
use std::ptr::hash;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::common::errors::{AppError, Result};

/// Hash password using Argon2 algorithm
pub fn hash_password(password: &str) -> Result<String> {
    // Generate random salt
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash password with salt
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::Internal(format("Failed to hash password: {}", e)))
}

/// verify password against stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // Parse stored hash
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::internal(format!("Invalid password hash: {}", e)))?;

    // Verify password matches hash
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}