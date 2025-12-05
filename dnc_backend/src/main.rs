mod db;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use dnc_backend::{build_app, AppState};
use db::check_db;
#[tokio::main]
async fn main(){
    dotenv().ok(); // Load environment variables from .env file
    let port=std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let the_state= AppState::new().await;
    check_db(&the_state.db).await;
    let app=build_app(the_state);
    let addr= SocketAddr::from(([0,0,0,0], port));
    let listener=TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    println!("Listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();

}