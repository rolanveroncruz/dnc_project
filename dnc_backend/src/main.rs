mod db;
#[allow(unused_imports, dead_code)]
mod entities;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
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

    // Create a rolling file appender (daily rotation)
    let file_appender = tracing_appender::rolling::daily("logs/", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Setup tracin subscriber: console + file
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info, axum=info" )),
        )
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
        )
        .with(
            fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true)
        )   // console
        .init();// file

    let the_state= AppState::new().await;
    let db = &the_state.db;
    check_db(&db).await;
    let app=build_app(the_state);
    let addr= SocketAddr::from(([0,0,0,0], port));
    let listener=TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();

    Ok(())
}