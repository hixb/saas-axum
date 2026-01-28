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

/// Handle user login
pub async fn login(
    db: &DatabaseConnection,
    req: LoginRequest,
    jwt_secret: &str,
    jwt_exp: i64,
) -> Result<AuthResponse> {
    // Query user
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&req.username))
        .one(db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    if !verify_password(&req.password, &user.password)? {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Check account status
    if user.status != 1 {
        return Err(AppError::Forbidden("Account is disabled".to_string()));
    }

    // Generate access token
    let claims = Claims::new_access_token(
        user.id,
        user.username.clone(),
        user.role_id.unwrap_or(0),
        jwt_exp,
    );
    let access_token = generate_token(&claims, jwt_secret)?;

    Ok(AuthResponse {
        access_token,
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
            status: user.status,
        },
    })
}

/// Handle user registration
pub async fn register(
    db: &DatabaseConnection,
    req: RegisterRequest,
) -> Result<i32> {
    // Check username uniqueness
    let existing_user = users::Entity::find()
        .filter(users::Column::Username.eq(&req.username))
        .one(db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    // Check email uniqueness
    let existing_email = users::Entity::find()
        .filter(users::Column::Email.eq(&req.email))
        .one(db)
        .await?;

    if existing_email.is_some() {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    // Get default role (assuming role_id 2 is for regular users)
    let default_role = roles::Entity::find_by_id(2)
        .one(db)
        .await?
        .ok_or_else(|| AppError::Internal("Default role not found".to_string()))?;

    // Hash password
    let hashed_password = hash_password(&req.password)?;

    // Create user
    let new_user = users::ActiveModel {
        username: Set(req.username),
        email: Set(req.email),
        nickname: Set(req.nickname),
        password: Set(hashed_password),
        role_id: Set(Some(default_role.id)),
        status: Set(1),
        ..Default::default()
    };

    let result = users::Entity::insert(new_user).exec(db).await?;

    Ok(result.last_insert_id)
}
