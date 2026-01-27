use dotenvy::dotenv;
use saas_axum::{common::{db, AppState}, create_router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "saas_axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Establish database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = db::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("✅ Database connected");

    // Load JWT configuration
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_expiration = std::env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "86400".to_string())
        .parse()
        .expect("JWT_EXPIRATION must be a number");

    // Create application state
    let state = AppState::new(db_conn, jwt_secret, jwt_expiration);

    // Build router with all routes
    let app = create_router(state);

    // Configure server address
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("🚀 Server listening on http://{}:{}", host, port);

    // Start HTTP server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}