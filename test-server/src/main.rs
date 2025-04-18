use std::{error::Error, time::Duration};

use log::debug;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if dotenvy::dotenv().is_err() {
        return Err("Failed to load .env file".into());
    }

    env_logger::init();

    let db = database_connection().await?;
    assert!(db.ping().await.is_ok());

    Ok(())
}

async fn database_connection() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db_url = dotenvy::var("DATABASE_URL")?;
    debug!("Connecting to database at {}", db_url);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;
    Ok(db)
}
