use crate::config::load_env::Config;
use crate::infra::cache::redis;
use crate::infra::database::postgres;
use crate::infra::messaging::mqtt::MqttManager;
use crate::modules::{handle_subscribe_message, init_routes};
use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::fmt;
pub async fn run() -> std::io::Result<()> {
    let config = Config::load();
    fmt().with_env_filter("info").init();
    info!("Connecting to database at {}", config.database_url);

    let redis = redis::establish_connection(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");
    let mqtt = Arc::new(
        init_mqtt(
            &config.mqtt_broker,
            &config.mqtt_port,
            &config.mqtt_client_id,
        )
        .await,
    );
    let mqtt_data = web::Data::from(mqtt.clone());
    let db_pool = postgres::establish_connection(config.database_url).await;
    info!(
        "Starting server at http://{}:{}",
        config.server_host, config.server_port
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .app_data(web::Data::new(mqtt_data.clone()))
    })
    .bind((
        config.server_host,
        config.server_port.parse::<u16>().unwrap(),
    ))?
    .run()
    .await
}

pub async fn init_mqtt(broker: &String, port: &String, client_id: &String) -> MqttManager {
    let mqtt_manager = MqttManager::new_with_dispatcher(
        broker,
        port.parse::<u16>().unwrap(),
        client_id,
        handle_subscribe_message,
    )
    .await;
    mqtt_manager.subscribe("system/#").await;
    mqtt_manager
}
