pub mod common;
pub mod config;
pub mod entity;
pub mod middleware;
pub mod modules;

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    common::AppState,
    middleware::auth_middleware,
    modules::{auth, user},
};

/// Create and configure application router with all routes
pub fn create_router(state: AppState) -> Router {
    // Configure CORS to allow all origins (restrict in production)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes accessible without authentication
    let public_routes = Router::new()
        .route("/auth/login", post(auth::handlers::login_handler))
        .route("/auth/register", post(auth::handlers::register_handler))
        .route("/health", get(health_check));

    // Protected routes requiring authentication
    let protected_routes = Router::new()
        .route("/users/me", get(user::handlers::get_current_user))
        .route("/users", get(user::handlers::list_users))
        .route_layer(from_fn_with_state(state.clone(), auth_middleware));

    // Combine all routes under /api prefix
    Router::new()
        .nest("/api", public_routes)
        .nest("/api", protected_routes)
        .layer(cors)
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}