mod handlers;

use axum::{
    routing::get,
    Router,
};

use std::net::SocketAddr;
use crate::handlers::boiler::hello_world;

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get( hello_world));

    // Define the address to serve the application on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}