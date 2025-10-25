use paho_mqtt as mqtt;
use std::time::Duration;

pub fn establish_connection(broker_url: &str, client_id: &str) -> Result<mqtt::Client, mqtt::Error> {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(broker_url)
        .client_id(client_id)
        .finalize();

    let client = mqtt::Client::new(create_opts)?;

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(30))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
        .finalize();

    client.connect(conn_opts)?;
    println!("Connected to MQTT broker: {}", broker_url);

    Ok(client)
}

pub async fn publish_message(
    client: &mqtt::Client,
    topic: &str,
    message: &str,
    qos: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = mqtt::Message::new(topic, message, qos);
    client.publish(msg)?;
    Ok(())
}