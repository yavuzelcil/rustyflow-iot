//! RustyFlow IoT Platform - MQTT Gateway
//!
//! MQTT broker'a bağlanıp sensör verilerini dinleyen gateway servisi.
//! - Mosquitto MQTT broker'a bağlanır
//! - Topic'leri subscribe eder (sensors/#, devices/# vb.)
//! - Gelen mesajları shared-types formatında parse eder
//! - İleride: API server'a forward edebilir

mod config;

use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};
use tokio::time::Duration;
use tracing::{info, warn, error, debug};
use config::Config;
use shared_types::messages::MqttMessage;
use reqwest::Client as HttpClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ========== 1. KONFIGURASYON ==========
    // .env dosyasından ayarları yükle
    let cfg = Config::load();

    // ========== 2. LOGGING SISTEMI ==========
    // Structured logging'i başlat
    tracing_subscriber::fmt()
        .with_env_filter(cfg.log_level.clone())
        .init();

    info!("🚀 MQTT Gateway starting...");
    info!("📡 Broker: {}:{}", cfg.mqtt_broker_host, cfg.mqtt_broker_port);
    info!("🔖 Client ID: {}", cfg.mqtt_client_id);

    // ========== 3. MQTT CLIENT OLUŞTUR ==========
    // MQTT bağlantı seçeneklerini ayarla
    let mut mqttoptions = MqttOptions::new(
        cfg.mqtt_client_id.clone(),
        cfg.mqtt_broker_host.clone(),
        cfg.mqtt_broker_port
    );
    
    // Keep-alive: 5 saniye (bağlantının canlı olduğunu kontrol et)
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    
    // Clean session: true (her başlangıçta temiz başla)
    mqttoptions.set_clean_session(true);

    // Async MQTT client ve event loop oluştur
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // ========== 4. TOPIC'LERE SUBSCRIBE OL ==========
    // Config'den topic listesini al
    let topics = cfg.parse_topics();
    info!("📬 Subscribing to {} topics:", topics.len());
    
    for topic in topics {
        info!("   → {}", topic);
        client.subscribe(&topic, QoS::AtMostOnce).await?;
    }

    info!("✅ Gateway ready, listening for messages...");

    // ========== 5. HTTP CLIENT ==========
    // API server'a veri göndermek için HTTP client oluştur
    let http_client = HttpClient::new();
    let api_url = std::env::var("API_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    let sensor_endpoint = format!("{}/api/sensors", api_url);
    info!("🌐 API server: {}", sensor_endpoint);

    // ========== 6. EVENT LOOP - MESAJLARI DİNLE ==========
    // MQTT broker'dan gelen tüm event'leri işle
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                debug!("📥 Event: {:?}", notification);
                
                // Sadece gelen mesajları işle (Publish event'leri)
                if let Event::Incoming(Packet::Publish(publish)) = notification {
                    handle_message(&publish.topic, &publish.payload, &http_client, &sensor_endpoint).await;
                }
            }
            Err(e) => {
                error!("❌ Connection error: {}", e);
                // Bağlantı hatası olursa 5 saniye bekle ve tekrar dene
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

/// Sensör verisi - API server'a gönderilecek format
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SensorData {
    device_id: String,
    sensor_type: String,
    value: f64,
    unit: String,
    timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<serde_json::Value>,
}

/// Gelen MQTT mesajını işle ve API server'a forward et
/// 
/// # Parametreler
/// - `topic`: Mesajın geldiği MQTT topic (örn: "sensors/edge-agent/temperature")
/// - `payload`: Mesaj içeriği (byte array)
/// - `http_client`: API server'a request göndermek için HTTP client
/// - `sensor_endpoint`: API server'ın sensor endpoint'i
/// 
/// # İşlem Adımları
/// 1. Payload'u String'e dönüştür
/// 2. JSON parse et (shared-types::MqttMessage formatında)
/// 3. SensorReading'i SensorData'ya çevir
/// 4. API server'a POST et
async fn handle_message(topic: &str, payload: &[u8], http_client: &HttpClient, sensor_endpoint: &str) {
    // Payload'u String'e çevir
    let payload_str = match std::str::from_utf8(payload) {
        Ok(s) => s,
        Err(e) => {
            warn!("⚠️  Invalid UTF-8 in payload from {}: {}", topic, e);
            return;
        }
    };

    info!("📨 Message on '{}': {}", topic, payload_str);

    // JSON parse et (shared-types::MqttMessage formatı)
    match serde_json::from_str::<MqttMessage>(payload_str) {
        Ok(msg) => {
            info!("✅ Parsed message:");
            info!("   Device ID: {}", msg.device_id);
            info!("   Message type: {:?}", msg.message_type);
            
            // SensorReading'i payload'dan parse et
            if let Ok(reading) = serde_json::from_value::<shared_types::sensor::SensorReading>(msg.payload.clone()) {
                // Sensör tipini topic'ten al
                let sensor_type = topic.split('/').last().unwrap_or("unknown").to_string();
                
                // String değeri f64'e çevir
                let value = reading.value.parse::<f64>().unwrap_or(0.0);
                
                // Unit'i sensör tipine göre belirle
                let unit = match sensor_type.as_str() {
                    "temperature" => "°C".to_string(),
                    "humidity" => "%".to_string(),
                    "motion" => "bool".to_string(),
                    _ => "".to_string(),
                };
                
                let sensor_data = SensorData {
                    device_id: msg.device_id.to_string(),
                    sensor_type,
                    value,
                    unit,
                    timestamp: reading.timestamp.to_rfc3339(),
                    metadata: reading.metadata.clone(),
                };

                debug!("📦 Sensor data to forward: {:?}", sensor_data);

                // API server'a POST request
                match http_client.post(sensor_endpoint)
                    .json(&sensor_data)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            info!("✅ Forwarded to API server: {}", sensor_data.sensor_type);
                        } else {
                            warn!("⚠️  API server returned error: {}", response.status());
                        }
                    }
                    Err(e) => {
                        error!("❌ Failed to forward to API server: {}", e);
                    }
                }
            } else {
                debug!("ℹ️  Payload is not a SensorReading");
            }
        }
        Err(e) => {
            // JSON parse başarısız (farklı format olabilir, sorun değil)
            debug!("ℹ️  Not a MqttMessage format: {} (raw: {})", e, payload_str);
        }
    }
}
