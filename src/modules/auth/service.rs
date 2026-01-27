use sea_orm::*;

use crate::{
    common::{
        errors::{AppError, Result},
        jwt::{generate_token, Claims},
        password::{hash_password, verify_password},
    },
    entity::{users, roles},
    modules::auth::dto::{AuthResponse, LoginRequest, RegisterRequest, UserInfo},
};

/// Handle user login authentication
pub async fn login(db: &DatabaseConnection, req: LoginRequest, jwt_secret: &str, jwt_exp: i64) -> Result<AuthResponse> {
    // Query user by username
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&req.username))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password matches stored hash
    if !verify_password(&req.password, &user.password)? {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Check if user account is active
    if user.status != 1 {
        return Err(AppError::Forbidden("Account is disabled".to_string()));
    }

    // Generate JWT token with user claims
    let claims = Claims::new(user.id, user.username.clone(), user.role_id.unwrap_or(0), jwt_exp);
    let token = generate_token(&claims, jwt_secret)?;

    // Build authentication response
    Ok(AuthResponse {
        access_token: token,
        refresh_token: None,
        token_type: "Bearer".to_string(),
        expires_in: jwt_exp,
        user: UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            nickname: user.nickname,
            avatar: user.avatar,
            role_id: user.role_id,
        },
    })
}

/// Handle new user registration
pub async fn register(db: &DatabaseConnection, req: RegisterRequest) -> Result<i32> {
    // Check if username already exists
    let existing_user = users::Entity::find()
        .filter(users::Column::Username.eq(&req.username))
        .one(db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    // Check if email already exists
    let existing_email = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(db)
        .await?;

    if existing_email.is_some() {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    // Get default role for new users (assuming role ID 2 is for regular users)
    let default_role = roles::Entity::find_by_id(2)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Internal("Default role not found".to_string()))?;

    // Hash password before storing
    let hashed_password = hash_password(&req.password)?;

    // Create new user record
    let new_user = users::ActiveModel {
        username: Set(req.username),
        email: Set(req.email),
        nickname: Set(req.nickname),
        password: Set(hashed_password),
        role_id: Set(Some(default_role.id)),
        status: Set(1),
        ..Default::default()
    };

    // Insert user into database
    let result = users::Entity::insert(new_user).exec(db).await?;

    Ok(result.last_insert_id)
}