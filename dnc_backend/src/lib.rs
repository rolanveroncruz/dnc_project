mod handlers;
mod db;
mod entities;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
use http::Method;
use tower_http::cors::CorsLayer;

impl AppState {
    pub async fn new() -> Self {
        let the_db = db::init_db().await.unwrap();
        Self { db: the_db }
    }
}


use axum::{Router, routing::get, routing::post};
use sea_orm::DatabaseConnection;
use handlers::boiler::{hello_world, healthcheck};
use handlers::user_roles_permissions::{ login_handler};
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};

pub fn build_app(my_state:AppState)->Router{

    // 1. Define CORS policy
    let cors = CorsLayer::new()
        // allow Angular dev origin
        .allow_origin("http://localhost:4200".parse::<http::HeaderValue>().unwrap())
        // allow methods
        .allow_methods(vec![Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::OPTIONS,])
        // allow headers frontend sends
        .allow_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE]);

    Router::new()
        .route("/hello", get( hello_world))
        .route("/healthcheck", get( healthcheck))
        .route("/login", post(login_handler))
        .with_state(my_state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .include_headers(true)
                    )
                .on_response(DefaultOnResponse::new()
                    .include_headers(true)
                    .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        )
}