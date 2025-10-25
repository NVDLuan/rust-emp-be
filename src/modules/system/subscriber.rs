use log::info;

pub fn handle_message(topic: String, payload: Vec<u8>) {
    let msg = String::from_utf8_lossy(&payload);
    info!("📩 [UserSubscriber] Received on {}: {}", topic, msg);

    if topic == "user/created" {
        info!("🧍 User created event handled!");
    } else if topic == "user/deleted" {
        info!("🧹 User deleted event handled!");
    }
}