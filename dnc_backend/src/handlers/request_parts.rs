use std::sync::Arc;
use crate::handlers::structs;
use axum::{
    extract::{FromRequestParts },
    http::{request::Parts, StatusCode},
};
use http::header::AUTHORIZATION;
use structs::{AuthUser, Claims, JwtConfig};
use jsonwebtoken::{decode  };

impl<S> FromRequestParts<S> for AuthUser
    where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection>{
        // If you use Axum state, prefer pulling config from State<AppState>.
        // Here is a simple pattern: downcast State via extension is not available,
        // so you’d normally do:
        // let State(app_state) = State::<AppState>::from_request_parts(parts, state).await?;

        // Instead, simplest is: store JwtConfig in request extensions via middleware,
        // OR implement FromRequestParts for your AppState and use it here.

        // --- Read Authorization header ---
        let auth = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

        let token = auth
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Expected Bearer token"))?;

        //---- Get JwtConfig from extensions (set by a layer) ----
        let cfg = parts
            .extensions
            .get::<Arc<JwtConfig>>()
            .cloned()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "JwtConfig not found in request extensions"))?;

        let data = decode::<Claims>(token, &cfg.decoding_key, &cfg.validation)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;

        Ok(AuthUser{claims:data.claims})
    }
}