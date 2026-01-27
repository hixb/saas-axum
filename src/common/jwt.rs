use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::common::errors::{AppError, Result};

/// JWT token claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// User ID (subject)
    pub sub: i32,
    /// Username for display
    pub username: String,
    /// User's role ID for authorization
    pub role_id: i32,
    /// Token expiration timestamp
    pub exp: i64,
    /// Token issued at timestamp
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: i32, username: String, role_id: i32, expiration_seconds: i64) -> Self {
        let now = Utc::now();
        let exp = (now + TimeDelta::seconds(expiration_seconds)).timestamp();

        Self {
            sub: user_id,
            username,
            role_id,
            exp,
            iat: now.timestamp(),
        }
    }
}

/// Generate JWT token from claims
pub fn generate_token(claims: &Claims, secret: &str) -> Result<String> {
    encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(AppError::from)
}

/// Verify and decode JWT token
pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
        .map(|data| data.claims)
        .map_err(AppError::from)
}