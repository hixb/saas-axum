use axum::{extract::State, Json};
use utoipa;
use validator::Validate;

use crate::{
    common::{errors::Result, response::success, AppState},
    modules::auth::{
        dto::{AuthResponse, LoginRequest, RegisterRequest},
        service,
    },
};

/// HTTP handler for login endpoint
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 422, description = "Validation error")
    ),
    tag = "Authentication"
)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<impl serde::Serialize>> {
    // Validate request payload
    payload.validate()
        .map_err(|e| crate::common::errors::AppError::ValidationError(e.to_string()))?;

    // Process login through service layer
    let response = service::login(&state.db, payload, &state.jwt_secret, state.jwt_expiration).await?;

    Ok(Json(success(response)))
}

/// HTTP handler for registration endpoint
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Registration successful"),
        (status = 409, description = "Username or email already exists"),
        (status = 422, description = "Validation error")
    ),
    tag = "Authentication"
)]
pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<impl serde::Serialize>> {
    // Validate request payload
    payload.validate()
        .map_err(|e| crate::common::errors::AppError::ValidationError(e.to_string()))?;

    // Process registration through service layer
    let user_id = service::register(&state.db, payload).await?;

    Ok(Json(success(serde_json::json!({
        "user_id": user_id,
        "message": "Registration successful"
    }))))
}