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
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    common::AppState,
    middleware::auth_middleware,
    modules::{auth, user},
};

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        auth::handlers::login_handler,
        auth::handlers::register_handler,
        user::handlers::get_current_user,
        user::handlers::list_users,
    ),
    components(
        schemas(
            auth::dto::LoginRequest,
            auth::dto::RegisterRequest,
            auth::dto::AuthResponse,
            auth::dto::UserInfo,
            user::dto::UserProfile,
            user::dto::UserListItem,
        )
    ),
    tags(
        (name = "Authentication", description = "Authentication endpoints for login and registration"),
        (name = "Users", description = "User management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}

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
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", public_routes)
        .nest("/api", protected_routes)
        .layer(cors)
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}