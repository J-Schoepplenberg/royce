use anyhow::Result;
use time::Duration;
use tower_sessions::{cookie::{SameSite, Key}, Expiry, MemoryStore, SessionManagerLayer};
use tower_sessions_redis_store::fred::{prelude::{ClientLike, RedisPool}, types::RedisConfig};

/// Setup the Redis connection pool.
/// 
/// Redis is used to store session data.
pub async fn setup_redis_store() -> Result<RedisPool> {
    let pool = RedisPool::new(RedisConfig::default(), None, None, None, 6)?;
    pool.wait_for_connect().await?;
    Ok(pool)
}