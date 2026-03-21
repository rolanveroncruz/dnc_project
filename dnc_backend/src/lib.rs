pub mod handlers;
pub use handlers::{LoginRequest, LoginResponse, Claims};
mod db;
mod entities;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
use axum::{extract::{Request, DefaultBodyLimit}, middleware::Next, response::Response};
use axum::{Router, routing::{get,post,put,patch}, middleware};

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
use axum::routing::delete;
use handlers::{require_jwt};
use crate::handlers::{get_data_objects, get_dental_service_types, post_dental_service, patch_dental_service};
use crate::handlers::{get_hmos, post_hmo, patch_hmo, get_hmo_by_id, post_dentist_contract, patch_dentist_contract};
use crate::handlers::{patch_dentist_contract_rates, get_regions, get_provinces, get_cities_by_province, get_cities};
use crate::handlers::{get_dental_clinics, get_dental_clinic_by_id, create_dental_clinic, patch_dental_clinic};
use crate::handlers::{get_clinic_capabilities_for_clinic, add_clinic_capability_to_clinic, remove_clinic_capability_from_clinic};
use crate::handlers::{set_clinic_capabilities_for_clinic, get_region_by_id, post_region, patch_region, get_all_dentists, get_dentist_names};
use crate::handlers::{get_dentist_from_id, get_clinics_for_dentist_id, get_all_dentist_clinics, get_dentists_for_clinic_id};
use crate::handlers::{get_all_dentist_histories, get_all_dentist_status, get_all_tax_classifications, get_all_tax_types};
use crate::handlers::{get_exclusive_to_hmos_from_dentist_id, get_not_hmos_from_dentist_id, add_dentist_clinic};
use crate::handlers::{remove_dentist_clinic, add_exclusive_to_hmo, remove_exclusive_to_hmo, add_except_for_hmo};
use crate::handlers::{remove_except_for_hmo, save_contract_file_for_dentist_id, get_contract_file_for_dentist_id};
use crate::handlers::{create_dentist, patch_dentist, get_all_account_types, get_dentist_clinic_positions};
use crate::handlers::{get_all_clinics_and_capabilities, get_endorsement_types, get_endorsement_billing_period_types};
use crate::handlers::{get_all_endorsements, create_endorsement, get_endorsement_by_id, patch_endorsement};
use crate::handlers::{get_endorsement_companies, post_endorsement_company, get_all_endorsement_rates, post_endorsement_rate};
use crate::handlers::{get_all_endorsement_counts, post_endorsement_count, put_endorsement_rate, patch_endorsement_rate};
use crate::handlers::{put_endorsement_count, patch_endorsement_count, upload_endorsement_master_list};
use crate::handlers::{get_master_list_meta_data_for_endorsement_id, delete_master_lists_for_endorsement_id};
use crate::handlers::{get_master_list_for_endorsement, set_master_list_member_active, get_endorsements_for_hmo_id};

use crate::handlers::{get_all_verifications};
use crate::handlers::{get_all_master_list_members, post_master_list_member, patch_master_list_member};

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
        .route("/hmos/{:id}/endorsements", get(get_endorsements_for_hmo_id))
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
        .route("/dental_clinics/{:clinic_id}/capabilities/", post(add_clinic_capability_to_clinic))
        .route("/dental_clinics/{:clinic_id}/capabilities/{:capability_id}", delete(remove_clinic_capability_from_clinic))
        .route("/dental/_clinics/{:clinic_id}/capabilities", patch(set_clinic_capabilities_for_clinic))
        .route("/bank_account_types", get(get_all_account_types))
        .route("/dentists/", get(get_all_dentists))
        .route("/dentist-names", get(get_dentist_names))
        .route("/dentists/{:id}", get(get_dentist_from_id))
        .route("/dentists/{:id}", patch(patch_dentist))

        .route("/dentist_clinics/positions", get(get_dentist_clinic_positions))

        .route("/dentist_clinics/", get(get_all_dentist_clinics))
        .route("/dentists/{:dentist_id}/clinics", get(get_clinics_for_dentist_id))
        .route("/dentists/{:dentist_id}/clinics", post(add_dentist_clinic))
        .route("/dentists/{:dentist_id}/clinics/{:clinic_id}", delete(remove_dentist_clinic))
        .route("/dental_clinics/{:clinic_id}/dentists", get(get_dentists_for_clinic_id))
        .route("/dentist_histories/", get(get_all_dentist_histories))
        .route("/dentist_statuses/", get(get_all_dentist_status))
        .route("/tax_classifications/", get(get_all_tax_classifications))
        .route("/tax_types/", get(get_all_tax_types))
        .route("/dentists/{:dentist_id}/hmos/exclusive", get(get_exclusive_to_hmos_from_dentist_id))
        .route("/dentists/{:dentist_id}/hmos/exclusive/{:hmo_id}", post(add_exclusive_to_hmo))
        .route("/dentists/{:dentist_id}/hmos/exclusive/{:hmo_id}", delete(remove_exclusive_to_hmo))
        .route("/dentists/{:dentist_id}/hmos/except", get(get_not_hmos_from_dentist_id))
        .route("/dentists/{:dentist_id}/hmos/except/{:hmo_id}", post(add_except_for_hmo))
        .route("/dentists/{:dentist_id}/hmos/except/{:hmo_id}", delete(remove_except_for_hmo))
        .route("/dentists/{:dentist_id}/contract-file", post(save_contract_file_for_dentist_id)
            .layer(DefaultBodyLimit::max(100 * 1024 * 1024)),)
        .route("/dentists/{:dentist_id}/contract-file/{:file_name}", get(get_contract_file_for_dentist_id))
        .route("/dentists/", post(create_dentist))
        .route("/extended_clinics", get(get_all_clinics_and_capabilities))
        .route("/endorsement_types", get(get_endorsement_types))
        .route("/endorsement_billing_period_types", get(get_endorsement_billing_period_types))
        .route("/endorsements", get(get_all_endorsements).post(create_endorsement))
        .route("/endorsements/{id}", get(get_endorsement_by_id).patch(patch_endorsement))
        .route("/endorsements/companies", get(get_endorsement_companies).post(post_endorsement_company))
        .route("/endorsements/{endorsement_id}/rates", get(get_all_endorsement_rates).post(post_endorsement_rate))
        .route("/endorsements/{endorsement_id}/rates/{rate_id}", put(put_endorsement_rate).patch(patch_endorsement_rate))
        .route("/endorsements/{endorsement_id}/counts", get(get_all_endorsement_counts).post(post_endorsement_count))
        .route("/endorsements/{endorsement_id}/counts/{count_id}", put(put_endorsement_count).patch(patch_endorsement_count))
        .route("/endorsements/{endorsement_id}/master_list", post(upload_endorsement_master_list))
        .route("/endorsements/{endorsement_id}/master_list_metadata", get(get_master_list_meta_data_for_endorsement_id))
        .route("/endorsements/{endorsement_id}/master_list", delete(delete_master_lists_for_endorsement_id).get(get_master_list_for_endorsement))
        .route("/endorsements/master_list_members/{master_list_member_id}/active", patch(set_master_list_member_active))
        .route("/verifications", get(get_all_verifications))
        .route("/master_list_members", get(get_all_master_list_members).post(post_master_list_member))
        .route("/master_list_members/{id}", patch(patch_master_list_member))

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
    tracing::info!("Allowed hosts: {:?}", allowed_hosts);

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
                            Method::PATCH,
                            Method::PUT,
                            Method::DELETE,
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