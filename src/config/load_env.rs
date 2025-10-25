use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub mqtt_broker: String,
    pub mqtt_client_id: String,
    pub auth_cookie: String,
    pub refresh_cookie: String,
    pub server_host: String,
    pub server_port: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();
        Self {
            database_url: Self::load_database_url(),
            redis_url: Self::load_redis_url(),
            mqtt_broker: Self::load_mqtt_broker(),
            mqtt_client_id: get_env_or_default("MQTT_CLIENT_ID", "server_receiver"),
            auth_cookie: get_env_or_default("AUTH_COOKIE", "auth_token"),
            refresh_cookie: get_env_or_default("REFRESH_COOKIE", "refresh_token"),
            server_host: get_env_or_default("SERVER_HOST", "0.0.0.0"),
            server_port: get_env_or_default("SERVER_PORT", "8080"),
        }
    }

    fn load_database_url() -> String {
        if let Ok(url) = env::var("DATABASE_URL") {
            return url;
        }

        let host = get_env("DB_HOST");
        let port = get_env("DB_PORT");
        let user = get_env("DB_USER");
        let password = get_env_or_default("DB_PASSWORD", "");
        let name = get_env("DB_NAME");

        if password.is_empty() {
            format!("postgres://{}@{}:{}/{}", user, host, port, name)
        } else {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                user, password, host, port, name
            )
        }
    }

    fn load_redis_url() -> String {
        if let Ok(url) = env::var("REDIS_URL") {
            return url;
        }

        let host = get_env_or_default("REDIS_HOST", "localhost");
        let port = get_env_or_default("REDIS_PORT", "6379");
        let password = env::var("REDIS_PASSWORD").ok();

        match password {
            Some(pass) if !pass.is_empty() => format!("redis://:{}@{}:{}", pass, host, port),
            _ => format!("redis://{}:{}", host, port), 
        }
    }

    fn load_mqtt_broker() -> String {
        if let Ok(url) = env::var("MQTT_BROKER") {
            return url;
        }

        let host = get_env_or_default("MQTT_HOST", "localhost");
        let port = get_env_or_default("MQTT_PORT", "1883");
        format!("tcp://{}:{}", host, port)
    }
}

fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("❌ Missing environment variable: {}", key))
}

fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
