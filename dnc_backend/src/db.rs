use log;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
#[allow(dead_code)]
pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public");

    let db = Database::connect(opt).await?;
    Ok(db)
}
#[allow(dead_code)]
pub async fn check_db(db: &DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    println!("Database connection OK");
}
