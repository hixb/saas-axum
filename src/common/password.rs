use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::common::errors::{AppError, Result};

/// Hash password using Argon2id algorithm (recommended by OWASP)
pub fn hash_password(password: &str) -> Result<String> {
    // Generate cryptographically secure random salt
    let salt = SaltString::generate(&mut OsRng);

    // Use Argon2id variant (hybrid of Argon2i and Argon2d)
    let argon2 = Argon2::default();

    // Hash password with salt
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))
}

/// Verify password against stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // Parse stored password hash
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash format: {}", e)))?;

    // Verify password matches hash
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Validate password strength
pub fn validate_password_strength(password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(AppError::ValidationError(
            "Password must be at least 8 characters long".to_string()
        ));
    }

    if password.len() > 128 {
        return Err(AppError::ValidationError(
            "Password must not exceed 128 characters".to_string()
        ));
    }

    // Check for at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one uppercase letter".to_string()
        ));
    }

    // Check for at least one lowercase letter
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one lowercase letter".to_string()
        ));
    }

    // Check for at least one digit
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one number".to_string()
        ));
    }

    // Check for at least one special character
    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err(AppError::ValidationError(
            "Password must contain at least one special character".to_string()
        ));
    }

    Ok(())
}