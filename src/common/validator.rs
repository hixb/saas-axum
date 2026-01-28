use regex::Regex;
use once_cell::sync::Lazy;
use validator::ValidationError;
use crate::common::errors::{AppError, Result};

/// Email validation regex pattern
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

/// Username validation regex pattern (alphanumeric and underscore only)
static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]{3,30}$").unwrap());

/// Phone number validation regex pattern (international format)
static PHONE_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap());

/// Validate email format
pub fn validate_email(email: &str) -> Result<()> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(AppError::ValidationError("Invalid email format".to_string()))
    }

    Ok(())
}

/// Validate username format
pub fn validate_username(username: &str) -> Result<()> {
    if !USERNAME_REGEX.is_match(username) {
        return Err(AppError::ValidationError(
            "Username must be 3-30 characters long and contain only letters, numbers, and underscores".to_string()
        ))
    }

    Ok(())
}

/// Validate phone number format
pub fn validate_phone(phone: &str) -> Result<()> {
    if !PHONE_NUMBER_REGEX.is_match(phone) {
        return Err(AppError::ValidationError("Invalid phone number format".to_string()))
    }

    Ok(())
}

/// Sanitize iser input to prevent XSS attacks
pub fn sanitize_input(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('&', "&amp;")
}