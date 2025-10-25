use redis::Client;
use redis::aio::ConnectionManager;

pub type RedisConnection = ConnectionManager;

pub async fn establish_connection(redis_url: &str) -> Result<RedisConnection, redis::RedisError> {
    let client = Client::open(redis_url)?;
    let manager = ConnectionManager::new(client)
        .await?;
    Ok(manager)
}
