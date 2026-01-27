//! Common utilities and infrastructure components

pub mod db;
pub mod errors;
pub mod jwt;
pub mod pagination;
pub mod password;
pub mod response;
pub mod state;

// Re-export commonly used types
pub use errors::{AppError, Result};
pub use pagination::PaginationParams;
pub use response::{success, success_with_message, ApiResponse, PaginatedResponse};
pub use state::AppState;