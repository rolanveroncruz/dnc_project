pub mod handlers;
pub use handlers::{LoginRequest, LoginResponse, Claims};
mod db;
mod entities;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

use std::time::Duration;
use http::{HeaderValue, Method, Request, Response};
use http::request::Parts;
use tower_http::cors::{CorsLayer, AllowOrigin};
use handlers::get_dental_services;
impl AppState {
    pub async fn new() -> Self {
        let the_db = db::init_db().await.unwrap();
        Self { db: the_db }
    }
}


use axum::{Router, routing::get, routing::post, middleware};
use sea_orm::DatabaseConnection;
use handlers::boiler::{hello_world, healthcheck, test_posting_json, whoami};
use handlers::user_roles_permissions::{ login_handler};
use tower_http::trace::{TraceLayer };
use tracing::Span;



fn protected_routes()->Router<AppState>{
    Router::<AppState>::new()
        .route("/test_post", post(test_posting_json))
        .route("/whoami", get(whoami))
        .route("/dental_services", get(get_dental_services))
}
use jsonwebtoken::{Validation, Algorithm, DecodingKey};
use handlers::JwtConfig;
use std::sync::Arc;
use handlers::{require_jwt};

pub fn build_app(my_state:AppState) ->Router{

    // 1. Define CORS policy
    let allowed_hosts : Vec<String> = std::env::var("ALLOW_HOSTS")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let cors = CorsLayer::new()
        // allow Angular dev origin
        .allow_origin(AllowOrigin::predicate( move |origin: &HeaderValue, _parts:&Parts | {
            allowed_hosts
                .iter()
                .any(|allowed| origin.as_bytes() == allowed.as_bytes())
        }))
             // allow methods
        .allow_methods(vec![Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::OPTIONS,])
        // allow headers frontend sends
        .allow_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE]);

    let mut validation = Validation::new(Algorithm::HS512);
    validation.validate_exp = true;

    let jwt_cfg = Arc::new(JwtConfig{
        decoding_key: DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes()),
        validation,
    });
    let protected:Router<AppState> = protected_routes().layer(middleware::from_fn_with_state(
        jwt_cfg.clone(),
        require_jwt,
    ));

    Router::new()
        .nest("/api", protected)
        .route("/hello", get( hello_world))
        .route("/healthcheck", get( healthcheck))
        .route("/login", post(login_handler))
        .with_state(my_state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .on_request(())
                .make_span_with( |req: &Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %req.method(),
                        path = %req.uri().path(),
                    )
                })
                .on_response( |res:&Response<_>, latency:Duration, span: &Span| {
                    let status = res.status().as_u16();
                    let latency_ms = latency.as_millis() as u64;
                    tracing::info!(
                        parent:span,
                        status = status,
                        latency_ms = latency_ms,
                        "request_finished"
                    );
                },
        ))
}