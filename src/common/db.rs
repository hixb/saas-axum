use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use tracing::log;

/// Establishes connection to PostgreSQL database with connection pooling
pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url);

    // Configure connection pool parameters
    opt.max_connections(100) // Maximum number of connections in pool
        .min_connections(5) // Minimum number of idle connections
        .connect_timeout(Duration::from_secs(8)) // Timeout for establishing connection
        .acquire_timeout(Duration::from_secs(8)) // Timeout for acquiring connection from pool
        .idle_timeout(Duration::from_secs(8)) // Timeout for idle connections
        .max_lifetime(Duration::from_secs(8)) // Maximum lifetime of a connection
        .sqlx_logging(true) // Enable SQL query logging
        .sqlx_logging_level(log::LevelFilter::Debug); // Set log level for SQL queries

    Database::connect(opt).await
}
