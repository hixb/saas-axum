use sea_orm::*;

use crate::{
    common::errors::{AppError, Result},
    entity::users,
    modules::user::dto::{UserListItem, UserProfile},
};

/// Get user profile by ID
pub async fn get_user_by_id(db: &DatabaseConnection, user_id: i32) -> Result<UserProfile> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(UserProfile {
        id: user.id,
        username: user.username,
        email: user.email,
        nickname: user.nickname,
        avatar: user.avatar,
        role_id: user.role_id,
        status: user.status,
    })
}

/// Get list of all users
pub async fn list_users(db: &DatabaseConnection) -> Result<Vec<UserListItem>> {
    let users = users::Entity::find().all(db).await?;

    Ok(users
        .into_iter()
        .map(|user| UserListItem {
            id: user.id,
            username: user.username,
            email: user.email,
            nickname: user.nickname,
            role_id: user.role_id,
            status: user.status,
        })
        .collect())
}
