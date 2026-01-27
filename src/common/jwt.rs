use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::common::errors::{AppError, Result};

/// JWT token claims structure with comprehensive user information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// User ID (subject)
    pub sub: i32,

    /// Username for display
    pub username: String,

    /// User's role ID for authorization
    pub role_id: i32,

    /// Token expiration timestamp (Unix timestamp)
    pub exp: i64,

    /// Token issued at timestamp (Unix timestamp)
    pub iat: i64,

    /// Token issuer
    pub iss: String,

    /// Token type (access or refresh)
    pub token_type: TokenType,
}

/// Token type enumeration
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

impl Claims {
    /// Create new JWT claims for access token
    pub fn new_access_token(
        user_id: i32,
        username: String,
        role_id: i32,
        expiration_seconds: i64,
    ) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::seconds(expiration_seconds)).timestamp();

        Self {
            sub: user_id,
            username,
            role_id,
            exp,
            iat: now.timestamp(),
            iss: "saas-axum".to_string(),
            token_type: TokenType::Access,
        }
    }

    /// Create new JWT claims for refresh token
    pub fn new_refresh_token(
        user_id: i32,
        username: String,
        role_id: i32,
        expiration_seconds: i64,
    ) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::seconds(expiration_seconds)).timestamp();

        Self {
            sub: user_id,
            username,
            role_id,
            exp,
            iat: now.timestamp(),
            iss: "saas-axum".to_string(),
            token_type: TokenType::Refresh,
        }
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    /// Check if token is access token
    pub fn is_access_token(&self) -> bool {
        self.token_type == TokenType::Access
    }

    /// Check if token is refresh token
    pub fn is_refresh_token(&self) -> bool {
        self.token_type == TokenType::Refresh
    }
}

/// Generate JWT token from claims
pub fn generate_token(claims: &Claims, secret: &str) -> Result<String> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
        .map_err(AppError::from)
}

/// Verify and decode JWT token
pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
        .map_err(AppError::from)?;

    // Check if token is expired
    if token_data.claims.is_expired() {
        return Err(AppError::Unauthorized("Token has expired".to_string()));
    }

    Ok(token_data.claims)
}

/// Verify access token specifically
pub fn verify_access_token(token: &str, secret: &str) -> Result<Claims> {
    let claims = verify_token(token, secret)?;

    if !claims.is_access_token() {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    Ok(claims)
}

/// Verify refresh token specifically
pub fn verify_refresh_token(token: &str, secret: &str) -> Result<Claims> {
    let claims = verify_token(token, secret)?;

    if !claims.is_refresh_token() {
        return Err(AppError::Unauthorized("Invalid token type".to_string()));
    }

    Ok(claims)
}