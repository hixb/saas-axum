use serde::Serialize;
use utoipa::ToSchema;

/// User profile response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfile {
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

    /// User account status: 1=active, 0=disabled
    #[schema(example = 1)]
    pub status: i32,
}

/// User list item
#[derive(Debug, Serialize, ToSchema)]
pub struct UserListItem {
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

    /// User role identifier for permissions (optional)
    #[schema(example = 1)]
    pub role_id: Option<i32>,

    /// User account status: 1=active, 0=disabled
    #[schema(example = 1)]
    pub status: i32,
}
