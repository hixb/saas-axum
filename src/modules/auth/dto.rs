use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Login request payload with validation rules
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    /// Username for authentication (3-255 characters)
    #[validate(length(min = 3, max = 255))]
    #[schema(example = "admin")]
    pub username: String,

    /// User password (minimum 6 characters)
    #[validate(length(min = 6))]
    #[schema(example = "password")]
    pub password: String,
}

/// User registration request with comprehensive validation
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    /// Unique username (3-30 characters, alphanumeric and underscore only)
    #[validate(length(min = 3, max = 30))]
    #[schema(example = "newuser")]
    pub username: String,

    /// Valid email address
    #[validate(email)]
    #[schema(example = "user@example.com")]
    pub email: String,

    /// Strong password (6-128 characters)
    #[validate(length(min = 6, max = 128))]
    #[schema(example = "password123")]
    pub password: String,

    /// Display name or full name (2-100 characters)
    #[validate(length(min = 2, max = 100))]
    #[schema(example = "John Doe")]
    pub nickname: String,
}

/// Token refresh request
#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    /// JWT refresh token for obtaining new access token
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
}

/// Authentication response with tokens and user info
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    /// JWT access token for API authentication
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,

    /// JWT refresh token for obtaining new access tokens (optional)
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: Option<String>,

    /// Token type, always "Bearer"
    #[schema(example = "Bearer")]
    pub token_type: String,

    /// Token expiration time in seconds
    #[schema(example = 3600)]
    pub expires_in: i64,

    /// Authenticated user information
    pub user: UserInfo,
}

/// User information included in auth response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    /// Unique user identifier
    #[schema(example = 1)]
    pub id: i32,

    /// Username for login
    #[schema(example = "admin")]
    pub username: String,

    /// User email address
    #[schema(example = "admin@example.com")]
    pub email: String,

    /// User display name
    #[schema(example = "Administrator")]
    pub nickname: String,

    /// User avatar URL (optional)
    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar: Option<String>,

    /// User role identifier for permissions (optional)
    #[schema(example = 1)]
    pub role_id: Option<i32>,

    /// User status: 1=active, 0=disabled
    #[schema(example = 1)]
    pub status: i32,
}

/// Password change request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    /// Current password for verification (minimum 6 characters)
    #[validate(length(min = 6))]
    #[schema(example = "oldPassword123")]
    pub old_password: String,

    /// New password (8-128 characters)
    #[validate(length(min = 8, max = 128))]
    #[schema(example = "NewPassword123!")]
    pub new_password: String,
}

/// Logout request
#[derive(Debug, Deserialize, ToSchema)]
pub struct LogoutRequest {
    /// Logout from all devices if true, current device only if false
    #[schema(example = false)]
    pub all_devices: Option<bool>,
}
