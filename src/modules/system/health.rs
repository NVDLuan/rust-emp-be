use actix_web::{web, HttpResponse, Result};
use crate::infra::database::postgres::DbPool;
use crate::infra::cache::redis::RedisConnection;
use redis::AsyncCommands;
use serde_json::json;

#[utoipa::path(
    get,
    path = "/health",
    tag = "System",
    responses(
        (status = 200, description = "Health check successful"),
        (status = 503, description = "Service unavailable")
    )
)]
pub async fn health_check(
    db: web::Data<DbPool>,
    redis: web::Data<RedisConnection>,
) -> Result<HttpResponse> {
    // Check database
    let db_healthy = db.ping().await.is_ok();
    
    // Check Redis - try a simple command to test connection
    let mut redis_conn = redis.get_ref().clone();
    let redis_healthy = redis_conn.get::<&str, Option<String>>("health_check").await.is_ok();
    
    let status = if db_healthy && redis_healthy {
        "healthy"
    } else {
        "unhealthy"
    };
    
    let status_code = if db_healthy && redis_healthy {
        actix_web::http::StatusCode::OK
    } else {
        actix_web::http::StatusCode::SERVICE_UNAVAILABLE
    };
    
    Ok(HttpResponse::build(status_code).json(json!({
        "status": status,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "services": {
            "database": if db_healthy { "up" } else { "down" },
            "redis": if redis_healthy { "up" } else { "down" }
        }
    })))
}
