use serde::Serialize;
use utoipa::ToSchema;

/// User profile response
#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfile {
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
    #[schema(example = 1)]
    pub status: i32,
}

/// User list item
#[derive(Debug, Serialize, ToSchema)]
pub struct UserListItem {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "admin")]
    pub username: String,
    #[schema(example = "admin@example.com")]
    pub email: String,
    #[schema(example = "Administrator")]
    pub nickname: String,
    #[schema(example = 1)]
    pub role_id: Option<i32>,
    #[schema(example = 1)]
    pub status: i32,
}