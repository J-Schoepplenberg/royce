use anyhow::Result;
use tokio::task::JoinHandle;
use tower_sessions_redis_store::fred::{
    error::RedisError,
    prelude::{ClientLike, RedisPool},
    types::{RedisConfig, Server, ServerConfig},
};

// Minimal Redis configuration.
fn redis_config() -> RedisConfig {
    RedisConfig {
        server: ServerConfig::Centralized {
            server: Server::new("redis", 6379),
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
