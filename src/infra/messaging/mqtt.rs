use rumqttc::{AsyncClient, MqttOptions, QoS, Incoming};
use std::sync::Arc;
use std::time::Duration;
use tokio::{task, sync::mpsc::{self, Sender}};
use log::{error};

#[derive(Clone)]
pub struct MqttManager {
    pub client: AsyncClient,
    pub tx: Sender<String>,
}

impl MqttManager {
    pub async fn new_with_dispatcher<F>(
        broker: &str,
        port: u16,
        client_id: &str,
        on_message: F,
    ) -> Self
    where
        F: Fn(String, Vec<u8>) + Send + Sync + 'static,
    {
        let callback = Arc::new(on_message);
        let mut options = MqttOptions::new(client_id, broker, port);
        options.set_keep_alive(Duration::from_secs(10));

        let (client, mut eventloop) = AsyncClient::new(options, 10);
        let (tx, mut rx) = mpsc::channel::<String>(10);

        // Task gửi tin
        let client_clone = client.clone();
        task::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = client_clone.publish("backend/pub", QoS::AtLeastOnce, false, msg).await {
                    error!("MQTT publish error: {:?}", e);
                }
            }
        });

        // Task nhận tin
        let callback_clone = callback.clone();
        task::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(rumqttc::Event::Incoming(Incoming::Publish(p))) => {
                        callback_clone(p.topic, p.payload.to_vec());
                    }
                    Err(e) => {
                        error!("MQTT event error: {:?}", e);
                        tokio::time::sleep(Duration::from_secs(3)).await;
                    }
                    _ => {}
                }
            }
        });

        Self { client, tx }
    }

    pub async fn publish(&self, topic: &str, payload: &str) {
        if let Err(e) = self.client.publish(topic, QoS::AtLeastOnce, false, payload).await {
            error!("MQTT publish failed: {:?}", e);
        }
    }

    pub async fn subscribe(&self, topic: &str) {
        if let Err(e) = self.client.subscribe(topic, QoS::AtMostOnce).await {
            error!("MQTT subscribe failed: {:?}", e);
        }
    }
}
