use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::common::{jwt::verify_token, AppState};

/// Middleware to verify JWT token and inject user claims into request
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header from request
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Parse Bearer token from header
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify token and extract claims
    let claims = verify_token(token, &state.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Inject claims into request extensions for downstream handlers
    req.extensions_mut().insert(claims);

    // Continue processing request
    Ok(next.run(req).await)
}