use anyhow::Result;
use std::env;
use tokio::task::JoinHandle;
use tower_sessions_redis_store::fred::{
    error::RedisError,
    prelude::{ClientLike, RedisPool},
    types::{RedisConfig, Server, ServerConfig},
};
use url::Url;

/// Parse the REDIS_URL environment variable and return a RedisConfig.
fn redis_config() -> RedisConfig {
    // Try to get the REDIS_URL from the environment. Provide a default if not found.
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://redis:6379".to_string());

    // Parse the URL. This will panic if the URL is not valid.
    let parsed_url = Url::parse(&redis_url)
        .expect("REDIS_URL must be a valid URL, e.g., redis://redis:6379");

    // Extract host and port from the URL. The host should always be present.
    let host = parsed_url.host_str().expect("No host found in REDIS_URL");
    let port = parsed_url.port().unwrap_or(6379);

    RedisConfig {
        server: ServerConfig::Centralized {
            server: Server::new(host, port),
        },
        ..Default::default()
    }
}

/// Setup the Redis connection pool.
pub async fn init_redis_store() -> Result<(RedisPool, JoinHandle<Result<(), RedisError>>)> {
    let config = redis_config();

    let redis_pool = RedisPool::new(config, None, None, None, 6)?;

    // Connects the connection pool to the Redis server.
    let redis_connection = redis_pool.connect();

    // Await that the whole pool is connected.
    redis_pool.wait_for_connect().await?;

    Ok((redis_pool, redis_connection))
}
