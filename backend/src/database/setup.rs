use crate::database::migration::run_migrations;
use anyhow::{Context, Result};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::env;
use tokio_postgres::{Config,NoTls};

/// Simplified type alias for a database connection pool.
pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

/// Initialize the database pool and run migrations.
pub async fn init_db() -> Result<ConnectionPool> {
    let pool = self::create_db_pool().await?;
    run_migrations().await?;
    Ok(pool)
}

/// Create database configuration from environment variables
fn create_db_config() -> Result<Config> {
    let mut config = Config::new();
    config
        .host(&env::var("POSTGRES_HOST").context("POSTGRES_HOST is not set")?)
        .port(env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()).parse()?)
        .user(&env::var("POSTGRES_USER").context("POSTGRES_USER is not set")?)
        .password(&env::var("POSTGRES_PASSWORD").context("POSTGRES_PASSWORD is not set")?)
        .dbname(&env::var("POSTGRES_DB").context("POSTGRES_DB is not set")?);
    Ok(config)
}

/// Create a database connection pool.
pub async fn create_db_pool() -> Result<ConnectionPool> {
    let config = create_db_config()?;
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::builder()
        .max_size(16)
        .build(manager)
        .await?;
    Ok(pool)
}

/// Retrieve the database configuration.
pub fn get_db_config() -> Result<tokio_postgres::Config> {
    // Use the same configuration as create_db_config
    create_db_config()
}

/// Retrieve a database client.
pub async fn get_db_client() -> Result<tokio_postgres::Client> {
    let cfg = get_db_config()?;
    let (client, con) = cfg.connect(tokio_postgres::NoTls).await?;
    // Spawn and poll the connection as a background task to run it concurrently.
    tokio::spawn(con);
    Ok(client)
}
