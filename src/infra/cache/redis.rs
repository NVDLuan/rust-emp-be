use redis::Client;
use redis::aio::ConnectionManager;

pub async fn establish_connection(redis_url: &str) -> Result<ConnectionManager, redis::RedisError> {
    let client = Client::open(redis_url)?;
    let manager = ConnectionManager::new(client)
        .await?;
    Ok(manager)
}
