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

    // ========== 5. EVENT LOOP - MESAJLARI DİNLE ==========
    // MQTT broker'dan gelen tüm event'leri işle
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                debug!("📥 Event: {:?}", notification);
                
                // Sadece gelen mesajları işle (Publish event'leri)
                if let Event::Incoming(Packet::Publish(publish)) = notification {
                    handle_message(&publish.topic, &publish.payload).await;
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

/// Gelen MQTT mesajını işle
/// 
/// # Parametreler
/// - `topic`: Mesajın geldiği MQTT topic (örn: "sensors/temperature")
/// - `payload`: Mesaj içeriği (byte array)
/// 
/// # İşlem Adımları
/// 1. Payload'u String'e dönüştür
/// 2. JSON parse et (shared-types::MqttMessage formatında)
/// 3. Mesaj tipine göre işle
/// 4. İleride: API server'a forward et
async fn handle_message(topic: &str, payload: &[u8]) {
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
            info!("   QoS: {:?}", msg.qos);
            
            // İleride buraya API server'a forward veya database write eklenebilir
        }
        Err(e) => {
            // JSON parse başarısız (farklı format olabilir, sorun değil)
            debug!("ℹ️  Not a MqttMessage format: {} (raw: {})", e, payload_str);
        }
    }
}
