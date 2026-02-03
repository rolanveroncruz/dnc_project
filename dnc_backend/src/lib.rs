pub mod handlers;
pub use handlers::{LoginRequest, LoginResponse, Claims};
mod db;
mod entities;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
use axum::{extract::Request, middleware::Next, response::Response};
use axum::{Router, routing::{get,post,patch}, middleware};
use sea_orm::DatabaseConnection;
use handlers::boiler::{hello_world, healthcheck, test_posting_json, whoami};
use handlers::login::{ login_handler};

use http::{HeaderValue, HeaderName, Method,};
use http::request::Parts;
use tower_http::cors::{CorsLayer, AllowOrigin};
use handlers::{
    get_dental_services,
    get_clinic_capabilities, post_clinic_capability, patch_clinic_capability,
    get_users, post_user, patch_user,
    get_roles,create_role,patch_role,
    get_role_permissions,
    get_all_dentist_contracts, get_dentist_contract,
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
use crate::handlers::{get_data_objects, get_dental_service_types, post_dental_service, patch_dental_service, get_hmos, post_hmo, patch_hmo, get_hmo_by_id, post_dentist_contract, patch_dentist_contract, patch_dentist_contract_rates, get_regions, get_provinces, get_cities_by_province, get_cities, get_dental_clinics, get_dental_clinic_by_id, create_dental_clinic, patch_dental_clinic, get_clinic_capabilities_for_clinic, add_clinic_capability_to_clinic, remove_clinic_capability_from_clinic, set_clinic_capabilities_for_clinic, get_region_by_id, post_region, patch_region };

fn protected_routes() ->Router<AppState>{
    Router::<AppState>::new()
        .route("/test_post", post(test_posting_json))
        .route("/whoami", get(whoami))
        .route("/dental_services", get(get_dental_services))
        .route("/dental_services/", post(post_dental_service))
        .route("/dental_services/{:id}", patch(patch_dental_service))
        .route("/dental_service_types", get(get_dental_service_types))
        .route("/clinic_capabilities", get(get_clinic_capabilities))
        .route("/clinic_capabilities/", post(post_clinic_capability))
        .route("/clinic_capabilities/{:id}", patch(patch_clinic_capability))
        .route("/users", get(get_users))
        .route("/users/", post(post_user))
        .route("/users/{:id}", patch(patch_user))
        .route("/roles", get(get_roles))
        .route("/roles/", post(create_role))
        .route("/roles/{:id}", patch(patch_role))
        .route("/role_permissions", get(get_role_permissions))
        .route("/data_objects", get(get_data_objects))
        .route("/hmos", get(get_hmos))
        .route("/hmos/{:id}", get(get_hmo_by_id))
        .route("/hmos/{:id}", patch(patch_hmo))
        .route("/hmos/", post(post_hmo))
        .route("/dentist_contracts",get(get_all_dentist_contracts))
        .route("/dentist_contracts/{:id}",get(get_dentist_contract))
        .route("/dentist_contracts/",post(post_dentist_contract))
        .route("/dentist_contracts/{:id}",patch(patch_dentist_contract))
        .route("/dentist_contracts/{:id}/rates",patch(patch_dentist_contract_rates))
        .route("/cities", get(get_cities))
        .route("/provinces", get(get_provinces))
        .route("/provinces/{:province_id}/cities", get(get_cities_by_province))
        .route("/regions", get(get_regions))
        .route("/regions/{:id}", get(get_region_by_id))
        .route("/regions/", post(post_region))
        .route("/regions/{:id}", patch(patch_region))
        .route("/dental_clinics/", get(get_dental_clinics))
        .route("/dental_clinics/{id}", get(get_dental_clinic_by_id))
        .route("/dental_clinics/", post(create_dental_clinic))
        .route("/dental_clinics/{id}", patch(patch_dental_clinic))
        .route("/dental_clinics/{:clinic_id}/capabilities", get(get_clinic_capabilities_for_clinic))
        .route("/dental/_clinics/{:clinic_id}/capabilities/", post(add_clinic_capability_to_clinic))
        .route("/dental/_clinics/{:clinic_id}/capabilities/{:capability_id}", patch(remove_clinic_capability_from_clinic))
        .route("/dental/_clinics/{:clinic_id}/capabilities", patch(set_clinic_capabilities_for_clinic))



}

async fn log_origin(req: Request, next: Next) -> Response {
    if let Some(o) = req.headers().get(http::header::ORIGIN) {
        tracing::info!("Origin: {:?}", o);
    } else {
        tracing::info!("Origin: <none>");
    }
    next.run(req).await
}

pub fn build_app(my_state:AppState) ->Router{

    // 1. Define CORS policy
    let allowed_hosts : Vec<String> = std::env::var("ALLOW_HOSTS")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.trim().to_string())
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
        .allow_headers(vec![
            http::header::AUTHORIZATION,
            http::header::CONTENT_TYPE,
            HeaderName::from_static("traceparent"),
            HeaderName::from_static("tracestate"),
            HeaderName::from_static("baggage"),
        ]);

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
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(middleware::from_fn(log_origin))
        .layer(cors)
}