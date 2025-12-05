mod handlers;
mod db;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        let the_db = db::init_db().await.unwrap();
        Self { db: the_db }
    }
}


use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use handlers::boiler::{hello_world, healthcheck};
pub fn build_app(my_state:AppState)->Router{
    Router::new()
        .route("/hello", get( hello_world))
        .route("/healthcheck", get( healthcheck))
        .with_state(my_state)
}