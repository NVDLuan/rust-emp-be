use actix_web::{web, App, HttpServer};
use emp_be::config::load_env::Config;
use emp_be::infra::cache::redis;
use emp_be::infra::database::postgres;
use emp_be::infra::messaging::mqtt;
use tracing::info;
use tracing_subscriber::fmt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load();
    fmt().with_env_filter("info").init();
    info!("Connecting to database at {}", config.database_url);

    let redis = redis::establish_connection(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");
    let db_pool = postgres::establish_connection(config.database_url).await;
    let mqtt_client = mqtt::establish_connection(&config.mqtt_broker, &config.mqtt_client_id)
        .expect("Failed to create MQTT client");
    info!(
        "Starting server at http://{}:{}",
        config.server_host, config.server_port
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .app_data(web::Data::new(mqtt_client.clone()))
    })
    .bind((
        config.server_host,
        config.server_port.parse::<u16>().unwrap(),
    ))?
    .run()
    .await
}
