mod handlers;
mod db;

use axum::{
    routing::get,
    Router,
};

use std::net::SocketAddr;
use crate::handlers::{
    boiler::hello_world,
    boiler::healthcheck,
};
use dotenvy::dotenv;
use db::{init_db,check_db};

#[tokio::main]
async fn main(){
    dotenv().ok(); // Load environment variables from .env file
    let _db = init_db().await.unwrap();

    check_db(&_db).await;
    println!("Database connection established");

    let server_port = std::env::var("SERVER_PORT").unwrap_or("3000".to_string());

    let app = Router::new()
        .route("/", get( hello_world))
        .route("/healthcheck", get( healthcheck));

    // Define the address to serve the application on
    let addr = SocketAddr::from(([127, 0, 0, 1], server_port.parse::<u16>().unwrap()));

    println!("Listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}