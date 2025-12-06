mod db;

use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use dnc_backend::{build_app, AppState};
use db::check_db;

use migration::{Migrator, MigratorTrait};
#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file

    let database_url=std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection=sea_orm::Database::connect(&database_url).await?;
    Migrator::up(&connection, None)
        .await
        .expect("Failed to run migrations!!");

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

    Ok(())
}