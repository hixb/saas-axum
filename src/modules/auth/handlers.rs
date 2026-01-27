use axum::{extract::State, Json};
use validator::Validate;

use crate::{
    common::{errors::Result, response::success, AppState},
    modules::auth::{
        dto::{LoginRequest, RegisterRequest},
        service,
    },
};

/// HTTP handler for login endpoint
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