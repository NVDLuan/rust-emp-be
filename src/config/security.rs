use std::env;

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiry_minutes: u64,
    pub refresh_token_expiry_days: u64,
    pub argon2_memory_cost: u32,
    pub argon2_time_cost: u32,
    pub argon2_parallelism: u32,
}

impl SecurityConfig {
    pub fn load() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set in environment variables"),
            jwt_expiry_minutes: env::var("JWT_EXPIRY_MINUTES")
                .unwrap_or_else(|_| "15".to_string())
                .parse()
                .expect("Invalid JWT_EXPIRY_MINUTES"),
            refresh_token_expiry_days: env::var("REFRESH_TOKEN_EXPIRY_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .expect("Invalid REFRESH_TOKEN_EXPIRY_DAYS"),
            argon2_memory_cost: env::var("ARGON2_MEMORY_COST")
                .unwrap_or_else(|_| "4096".to_string())
                .parse()
                .expect("Invalid ARGON2_MEMORY_COST"),
            argon2_time_cost: env::var("ARGON2_TIME_COST")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .expect("Invalid ARGON2_TIME_COST"),
            argon2_parallelism: env::var("ARGON2_PARALLELISM")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .expect("Invalid ARGON2_PARALLELISM"),
        }
    }
}
