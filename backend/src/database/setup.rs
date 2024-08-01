use crate::database::migration::run_migrations;
use anyhow::{Context, Result};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::{env, str::FromStr};
use tokio_postgres::NoTls;

/// Simplified type alias for a database connection pool.
pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

/// Initialize the database pool and run migrations.
pub async fn init_db() -> Result<ConnectionPool> {
    let pool = self::create_db_pool().await?;
    run_migrations().await?;
    Ok(pool)
}

/// Create a database connection pool.
pub async fn create_db_pool() -> Result<ConnectionPool> {
    let database_url = env::var("DB_URL").context("DB_URL is not set.")?;
    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)?;
    let pool = Pool::builder().build(manager).await?;
    Ok(pool)
}

/// Retireve the database configuration.
pub fn get_db_config() -> tokio_postgres::Config {
    let db_url = env::var("DB_URL").expect("DB_URL is not set.");
    tokio_postgres::Config::from_str(&db_url).unwrap()
}

/// Retrieve a database client.
pub async fn get_db_client() -> tokio_postgres::Client {
    let cfg = get_db_config();
    let (client, con) = cfg.connect(tokio_postgres::NoTls).await.unwrap();
    // Spawn and poll the connection as a background task to run it concurrently.
    tokio::spawn(con);
    client
}
