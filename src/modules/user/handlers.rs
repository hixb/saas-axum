use axum::{extract::State, Extension, Json};

use crate::{
    common::{errors::Result, jwt::Claims, response::success, AppState},
    modules::user::{dto::{UserProfile, UserListItem}, service},
};

/// Get current authenticated user profile
#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserProfile),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<impl serde::Serialize>> {
    // Extract user ID from JWT claims
    let user = service::get_user_by_id(&state.db, claims.sub).await?;

    Ok(Json(success(user)))
}

/// Get list of all users (admin only)
#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserListItem>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<impl serde::Serialize>> {
    let users = service::list_users(&state.db).await?;

    Ok(Json(success(users)))
}