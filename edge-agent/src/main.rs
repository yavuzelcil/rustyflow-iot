//! RustyFlow IoT Platform - Edge Agent
//!
//! Raspberry Pi veya diğer edge cihazlarda çalışan IoT agent.
//! - Mock sensörlerden veri okur (temperature, humidity, motion)
//! - MQTT broker'a periyodik olarak veri gönderir
//! - shared-types formatında mesaj üretir
//! - Gerçek sensörler için rppal veya embedded-hal kullanılabilir

mod config;
mod sensors;

use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::time::{interval, Duration};
use tracing::{info, warn, error};
use config::Config;
use sensors::SensorController;
use shared_types::messages::MqttMessage;
use chrono::Utc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ========== 1. KONFIGURASYON ==========
    let cfg = Config::load();

    // ========== 2. LOGGING ==========
    tracing_subscriber::fmt()
        .with_env_filter(cfg.log_level.clone())
        .init();

    info!("🤖 Edge Agent starting...");
    info!("📱 Device: {} ({})", cfg.device_name, cfg.device_id);
    info!("📡 MQTT Broker: {}:{}", cfg.mqtt_broker_host, cfg.mqtt_broker_port);
    info!("⏱️  Sensor interval: {} seconds", cfg.sensor_interval_secs);

    // ========== 3. MQTT CLIENT ==========
    let client_id = format!("edge-{}", cfg.device_id);
    let mut mqttoptions = MqttOptions::new(
        client_id,
        cfg.mqtt_broker_host.clone(),
        cfg.mqtt_broker_port
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_clean_session(true);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // ========== 4. SENSÖR CONTROLLER ==========
    let mut sensors = SensorController::new();
    info!("🔧 Initialized {} mock sensors", 3);

    // ========== 5. EVENT LOOP ==========
    // MQTT connection handling task
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(_) => {},
                Err(e) => {
                    error!("MQTT connection error: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });

    // ========== 6. SENSOR DATA LOOP ==========
    let mut timer = interval(Duration::from_secs(cfg.sensor_interval_secs));
    let device_id = cfg.device_id;
    let device_name = cfg.device_name.clone();

    info!("✅ Edge agent ready, starting sensor readings...");

    loop {
        timer.tick().await;

        // Tüm sensörlerden veri oku
        let sensor_data = sensors.read_all();
        
        info!("📊 Read {} sensor values:", sensor_data.len());
        for data in &sensor_data {
            info!("   • {} ({}): {} {}", 
                data.sensor_type,
                data.reading.sensor_id, 
                data.reading.value, 
                data.unit
            );
        }

        // Her sensör için ayrı MQTT mesajı gönder
        for data in sensor_data {
            let topic = format!("sensors/{}/{}", device_name, data.sensor_type);
            
            // MqttMessage formatında payload oluştur
            let message = MqttMessage {
                message_type: format!("{}_reading", data.sensor_type),
                payload: serde_json::to_value(&data.reading).unwrap_or_default(),
                timestamp: Utc::now(),
                device_id,
                qos: 0,
            };

            // JSON serialize et
            match serde_json::to_string(&message) {
                Ok(json) => {
                    // MQTT'ye publish et
                    if let Err(e) = client.publish(&topic, QoS::AtMostOnce, false, json.as_bytes()).await {
                        warn!("Failed to publish to {}: {}", topic, e);
                    } else {
                        info!("📤 Published to '{}'", topic);
                    }
                }
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                }
            }
        }

        info!("---");
    }
}
