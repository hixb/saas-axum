use serde::{Deserialize, Serialize};
use validator::Validate;

/// Login request payload
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 255))]
    pub username: String,

    #[validate(length(min = 6))]
    pub password: String,
}

/// User registration request payload
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 255))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6))]
    pub password: String,

    #[validate(length(min = 2, max = 100))]
    pub nickname: String,
}

/// Authentication response with token and user info
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

/// User information included in auth response
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub role_id: Option<i32>,
}