use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::distr::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::config::security::SecurityConfig;
use crate::errors::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User email
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate_access_token(email: &str) -> AppResult<String> {
    let config = SecurityConfig::load();
    let now = Utc::now();
    let expiration = now
        .checked_add_signed(Duration::minutes(config.jwt_expiry_minutes as i64))
        .ok_or_else(|| AppError::Internal("Invalid timestamp calculation".to_string()))?
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(config.jwt_secret.as_bytes())
    ).map_err(|e| AppError::Jwt(format!("JWT encoding failed: {}", e)))
}

pub fn generate_refresh_token() -> String {
    let mut rng = rand::rng();
    let refresh_token: String = (0..64)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    refresh_token
}

