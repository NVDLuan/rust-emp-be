use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("JWT error: {0}")]
    Jwt(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_code) = match self {
            AppError::Authentication(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "AUTH_ERROR"),
            AppError::Validation(_) => (actix_web::http::StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            AppError::Unauthorized(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            AppError::Forbidden(_) => (actix_web::http::StatusCode::FORBIDDEN, "FORBIDDEN"),
            AppError::NotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::Database(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            AppError::Redis(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "REDIS_ERROR"),
            AppError::Jwt(_) => (actix_web::http::StatusCode::UNAUTHORIZED, "JWT_ERROR"),
            AppError::Internal(_) => (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        HttpResponse::build(status).json(json!({
            "success": false,
            "message": self.to_string(),
            "error_code": error_code,
            "details": None::<String>
        }))
    }
}

pub type AppResult<T> = Result<T, AppError>;
