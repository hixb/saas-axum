use serde::Serialize;

/// User profile response
#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub role_id: Option<i32>,
    pub status: i32,
}

/// User list item
#[derive(Debug, Serialize)]
pub struct UserListItem {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub nickname: String,
    pub role_id: Option<i32>,
    pub status: i32,
}