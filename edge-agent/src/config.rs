//! Edge Agent Yapılandırması
//!
//! Device ID, MQTT broker bilgileri ve sensör ayarları.

use serde::Deserialize;
use uuid::Uuid;

/// Edge Agent yapılandırması
/// 
/// .env dosyasından veya ortam değişkenlerinden okunacak ayarlar.
/// 
/// Örnek .env dosyası:
/// ```ignore
/// DEVICE_ID=550e8400-e29b-41d4-a716-446655440000
/// DEVICE_NAME=raspberry-pi-01
/// MQTT_BROKER_HOST=localhost
/// MQTT_BROKER_PORT=1883
/// SENSOR_INTERVAL_SECS=5
/// RUST_LOG=info
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Bu cihazın benzersiz ID'si (UUID)
    /// 
    /// Her edge agent'ın farklı bir ID'si olmalı.
    /// Eğer ayarlanmazsa, her başlangıçta yeni UUID üretilir.
    #[serde(default = "generate_device_id")]
    pub device_id: Uuid,

    /// Cihaz adı (human-readable)
    /// 
    /// Varsayılan: "edge-agent"
    /// 
    /// Örnek: `DEVICE_NAME=rpi-kitchen`
    #[serde(default = "default_device_name")]
    pub device_name: String,

    /// MQTT broker adresi
    /// 
    /// Varsayılan: "localhost"
    #[serde(default = "default_broker_host")]
    pub mqtt_broker_host: String,

    /// MQTT broker portu
    /// 
    /// Varsayılan: 1883
    #[serde(default = "default_broker_port")]
    pub mqtt_broker_port: u16,

    /// Sensör okuma aralığı (saniye)
    /// 
    /// Varsayılan: 5 saniye
    /// 
    /// Her N saniyede bir mock sensör verisi üretilir.
    #[serde(default = "default_sensor_interval")]
    pub sensor_interval_secs: u64,

    /// Logging seviyesi
    /// 
    /// Varsayılan: "info"
    #[serde(default = "default_log")]
    pub log_level: String,
}

// Varsayılan değer fonksiyonları
fn generate_device_id() -> Uuid { Uuid::new_v4() }
fn default_device_name() -> String { "edge-agent".into() }
fn default_broker_host() -> String { "localhost".into() }
fn default_broker_port() -> u16 { 1883 }
fn default_sensor_interval() -> u64 { 5 }
fn default_log() -> String { "info".into() }

impl Config {
    /// Yapılandırmayı yükle
    pub fn load() -> Self {
        // .env dosyasını yükle
        let _ = dotenvy::dotenv();

        // Ortam değişkenlerini parse et
        let mut cfg: Config = envy::from_env().unwrap_or_else(|_| Config {
            device_id: generate_device_id(),
            device_name: default_device_name(),
            mqtt_broker_host: default_broker_host(),
            mqtt_broker_port: default_broker_port(),
            sensor_interval_secs: default_sensor_interval(),
            log_level: default_log(),
        });

        // RUST_LOG özel işlemi
        if let Ok(level) = std::env::var("RUST_LOG") {
            cfg.log_level = level;
        }

        cfg
    }
}
