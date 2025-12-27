use std::sync::Arc;
use axum::{
    middleware::Next,
    response::Response
};
use axum::extract::{Request, State};
use http::header::AUTHORIZATION;
use http::StatusCode;
use crate::Claims;
use crate::handlers::JwtConfig;

/// Middleware to inject the JWT config into the request extensions
pub async fn inject_jwt_config(
    State(cfg): State<Arc<JwtConfig>>,
    mut req: Request,
    next: Next,
) -> Response {
    req.extensions_mut().insert(cfg);
    next.run(req).await
}


use jsonwebtoken::{decode};
use crate::handlers::structs::AuthUser;

/// Middleware to check the presence of a JWT token in the Authorization header.
pub async fn require_jwt(
    State(cfg): State<Arc<JwtConfig>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    req.extensions_mut().insert(cfg.clone());
    let auth = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let data = decode::<Claims>(token, &cfg.decoding_key, &cfg.validation)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(AuthUser { claims: data.claims });
    Ok(next.run(req).await)
}