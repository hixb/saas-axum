use sea_orm::DatabaseConnection;

/// Global application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: DatabaseConnection,

    /// JWT secret for token signing/verification
    pub jwt_secret: String,

    /// JWT token expiration in seconds
    pub jwt_expiration: i64,

    /// Refresh token expiration in seconds
    pub refresh_token_expiration: i64,
}

impl AppState {
    /// Create new application state instance
    pub fn new(
        db: DatabaseConnection,
        jwt_secret: String,
        jwt_expiration: i64,
        refresh_token_expiration: i64,
    ) -> Self {
        Self {
            db,
            jwt_secret,
            jwt_expiration,
            refresh_token_expiration,
        }
    }
}