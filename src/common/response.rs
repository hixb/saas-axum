use std::str::pattern::Searcher;
use serde::Serialize;
use axum::{response::IntoResponse, Json};

/// Standard API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>
}

impl<T: Serialize> ApiResponse<T> {
    /// Create successful response with data
    pub fn success (data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None
        }
    }

    /// Create successful response with data and message
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message)
        }
    }
}

/// Convert ApiResponse into HTTP response
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Helper function to create success response
pub fn success<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse::success(data)
}