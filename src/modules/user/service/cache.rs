use redis::AsyncCommands;
use crate::infra::cache::redis::RedisConnection;
use crate::modules::user::schemas::schemas::UserResponse;
use crate::errors::{AppError, AppResult};
use serde_json;

pub struct UserCacheService {
    redis: RedisConnection,
}

impl UserCacheService {
    pub fn new(redis: RedisConnection) -> Self {
        Self { redis }
    }
    
    pub async fn cache_user(&self, user_id: &str, user: &UserResponse) -> AppResult<()> {
        let key = format!("user:{}", user_id);
        let serialized = serde_json::to_string(user)
            .map_err(|e| AppError::Internal(format!("Failed to serialize user: {}", e)))?;
        let mut conn = self.redis.clone();
        let _: () = conn.set_ex(key, serialized, 3600).await?; // Cache for 1 hour
        Ok(())
    }
    
    pub async fn get_cached_user(&self, user_id: &str) -> AppResult<Option<UserResponse>> {
        let key = format!("user:{}", user_id);
        let mut conn = self.redis.clone();
        let cached: Option<String> = conn.get(key).await?;
        
        if let Some(data) = cached {
            let user: UserResponse = serde_json::from_str(&data)
                .map_err(|e| AppError::Internal(format!("Failed to deserialize user: {}", e)))?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
    
    pub async fn cache_user_by_email(&self, email: &str, user: &UserResponse) -> AppResult<()> {
        let key = format!("user:email:{}", email);
        let serialized = serde_json::to_string(user)
            .map_err(|e| AppError::Internal(format!("Failed to serialize user: {}", e)))?;
        let mut conn = self.redis.clone();
        let _: () = conn.set_ex(key, serialized, 3600).await?; // Cache for 1 hour
        Ok(())
    }
    
    pub async fn get_cached_user_by_email(&self, email: &str) -> AppResult<Option<UserResponse>> {
        let key = format!("user:email:{}", email);
        let mut conn = self.redis.clone();
        let cached: Option<String> = conn.get(key).await?;
        
        if let Some(data) = cached {
            let user: UserResponse = serde_json::from_str(&data)
                .map_err(|e| AppError::Internal(format!("Failed to deserialize user: {}", e)))?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
    
    pub async fn invalidate_user_cache(&self, user_id: &str, email: &str) -> AppResult<()> {
        let mut conn = self.redis.clone();
        
        // Remove both ID and email based cache entries
        let id_key = format!("user:{}", user_id);
        let email_key = format!("user:email:{}", email);
        
        let _: () = conn.del(vec![id_key, email_key]).await?;
        Ok(())
    }
    
    pub async fn cache_token_blacklist(&self, token: &str, expiry_seconds: u64) -> AppResult<()> {
        let key = format!("blacklist:token:{}", token);
        let mut conn = self.redis.clone();
        let _: () = conn.set_ex(key, "1", expiry_seconds).await?;
        Ok(())
    }
    
    pub async fn is_token_blacklisted(&self, token: &str) -> AppResult<bool> {
        let key = format!("blacklist:token:{}", token);
        let mut conn = self.redis.clone();
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }
}
