use serde::Serialize;
use axum::{response::IntoResponse, Json};

/// Standard API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: u16,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: String,
}

/// Paginated response wrapper
#[derive(Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub code: u16,
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
    timestamp: String
}

/// Pagination metadata
#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u64,
    pub page_size: u64,
    pub total_items: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool
}

impl<T: Serialize> ApiResponse<T> {
    /// Create successful response with data
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create successful response with data and message
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: Some(message),
            timestamp: chrono::Utc::now().to_rfc3339(),
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

/// Helper function to create success response with message
pub fn success_with_message<T: Serialize>(data: T, message: impl Into<String>) -> ApiResponse<T> {
    ApiResponse::success_with_message(data, message.into())
}

impl<T: Serialize> PaginatedResponse<T>{
    /// Create paginated response
    pub fn new(data: Vec<T>, page: u64, page_size: u64, total_items: u64) -> Self {
        let total_pages = (total_items + page_size - 1) / page_size;

        Self {
            code: 200,
            data,
            pagination: PaginationMeta {
                page,
                page_size,
                total_items,
                total_pages,
                has_next: page < total_pages,
                has_prev: page > 1
            },
            timestamp: chrono::Utc::now().to_rfc3339()
        }
    }
}

impl<T: Serialize> IntoResponse for PaginatedResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}