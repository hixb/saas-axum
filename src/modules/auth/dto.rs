use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Login request payload
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 255))]
    #[schema(example = "admin")]
    pub username: String,

    #[validate(length(min = 6))]
    #[schema(example = "password")]
    pub password: String,
}

/// User registration request payload
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 255))]
    #[schema(example = "newuser")]
    pub username: String,

    #[validate(email)]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[validate(length(min = 6))]
    #[schema(example = "password123")]
    pub password: String,

    #[validate(length(min = 2, max = 100))]
    #[schema(example = "John Doe")]
    pub nickname: String,
}

/// Authentication response with token and user info
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    pub refresh_token: Option<String>,
    #[schema(example = "Bearer")]
    pub token_type: String,
    #[schema(example = 3600)]
    pub expires_in: i64,
    pub user: UserInfo,
}

/// User information included in auth response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "admin")]
    pub username: String,
    #[schema(example = "admin@example.com")]
    pub email: String,
    #[schema(example = "Administrator")]
    pub nickname: String,
    pub avatar: Option<String>,
    #[schema(example = 1)]
    pub role_id: Option<i32>,
}