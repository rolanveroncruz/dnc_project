pub mod handlers;
pub use handlers::{LoginRequest, LoginResponse, Claims};
mod db;
mod entities;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

use axum::{Router, routing::{get,post,patch}, middleware};
use sea_orm::DatabaseConnection;
use handlers::boiler::{hello_world, healthcheck, test_posting_json, whoami};
use handlers::login::{ login_handler};

use http::{HeaderValue, Method,};
use http::request::Parts;
use tower_http::cors::{CorsLayer, AllowOrigin};
use handlers::{
    get_dental_services,
    get_clinic_capabilities,
    get_users,
    get_roles,create_role,patch_role,
    get_role_permissions
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
impl AppState {
    pub async fn new() -> Self {
        let the_db = db::init_db().await.unwrap();
        Self { db: the_db }
    }
}


use jsonwebtoken::{Validation, Algorithm, DecodingKey};
use handlers::JwtConfig;
use std::sync::Arc;
use handlers::{require_jwt};
use crate::handlers::{get_data_objects, get_dental_service_types, get_hmos, get_hmo_by_id};

fn protected_routes() ->Router<AppState>{
    Router::<AppState>::new()
        .route("/test_post", post(test_posting_json))
        .route("/whoami", get(whoami))
        .route("/dental_services", get(get_dental_services))
        .route("/dental_service_types", get(get_dental_service_types))
        .route("/clinic_capabilities", get(get_clinic_capabilities))
        .route("/users", get(get_users))
        .route("/roles", get(get_roles))
        .route("/roles/", post(create_role))
        .route("/roles/{:id}", patch(patch_role))
        .route("/role_permissions", get(get_role_permissions))
        .route("/data_objects", get(get_data_objects))
        .route("/hmos", get(get_hmos))
        .route("/hmos/{:id}", get(get_hmo_by_id))
}

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
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
}