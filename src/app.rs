use crate::config::load_env::Config;
use crate::config::logging;
use crate::infra::cache::redis;
use crate::infra::database::postgres;
use crate::infra::messaging::mqtt::MqttManager;
use crate::modules::{handle_subscribe_message, init_routes, swagger_routes};
use crate::errors::AppError;
use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use tracing::info;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging();
    
    let config = Config::load();
    info!("🔧 Configuration loaded successfully");
    info!("🔗 Connecting to database at {}", config.database_url);

    let redis = redis::establish_connection(&config.redis_url)
        .await
        .map_err(|e| {
            tracing::error!("❌ Failed to connect to Redis: {}", e);
            AppError::Redis(e)
        })?;
    info!("✅ Redis connection established successfully");
        
    let mqtt = Arc::new(
        init_mqtt(
            &config.mqtt_broker,
            &config.mqtt_port,
            &config.mqtt_client_id,
        )
        .await,
    );
    let mqtt_data = web::Data::from(mqtt.clone());
    info!("✅ MQTT connection established successfully");
    
    let db_pool = postgres::establish_connection(config.database_url.clone()).await
        .map_err(|e| {
            tracing::error!("❌ Failed to connect to database: {}", e);
            e
        })?;
    info!("✅ Database connection established successfully");
        
    info!(
        "🚀 Starting server at http://{}:{}",
        config.server_host, config.server_port
    );
    
    let server_port = config.server_port.parse::<u16>()
        .map_err(|e| {
            tracing::error!("Invalid server port: {}", e);
            AppError::Internal(format!("Invalid server port: {}", e))
        })?;
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .app_data(web::Data::new(mqtt_data.clone()))
            .configure(init_routes)
            .configure(swagger_routes)
    })
    .bind((config.server_host, server_port))?
    .run()
    .await
    .map_err(|e| {
        tracing::error!("Server failed to start: {}", e);
        e.into()
    })
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
