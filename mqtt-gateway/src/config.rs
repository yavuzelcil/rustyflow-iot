//! MQTT Gateway Yapılandırması
//!
//! MQTT broker bağlantı bilgileri ve gateway ayarları.

use serde::Deserialize;

/// MQTT Gateway yapılandırması
/// 
/// .env dosyasından veya ortam değişkenlerinden okunacak ayarlar.
/// 
/// Örnek .env dosyası:
/// ```ignore
/// MQTT_BROKER_HOST=localhost
/// MQTT_BROKER_PORT=1883
/// MQTT_CLIENT_ID=rustyflow-gateway
/// MQTT_TOPICS=sensors/#,devices/#
/// RUST_LOG=info
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// MQTT broker adresi
    /// 
    /// Varsayılan: "localhost"
    /// 
    /// Örnek: `MQTT_BROKER_HOST=mqtt.example.com`
    #[serde(default = "default_broker_host")]
    pub mqtt_broker_host: String,

    /// MQTT broker portu
    /// 
    /// Varsayılan: 1883 (MQTT standart port)
    /// 
    /// Örnek: `MQTT_BROKER_PORT=1883`
    #[serde(default = "default_broker_port")]
    pub mqtt_broker_port: u16,

    /// MQTT client ID
    /// 
    /// Broker'a bağlanırken kullanılacak benzersiz isim.
    /// 
    /// Varsayılan: "rustyflow-gateway"
    /// 
    /// Örnek: `MQTT_CLIENT_ID=gateway-001`
    #[serde(default = "default_client_id")]
    pub mqtt_client_id: String,

    /// Dinlenecek MQTT topic'leri (virgülle ayrılmış)
    /// 
    /// Wildcard destekler: # (tüm alt seviyeler), + (tek seviye)
    /// 
    /// Varsayılan: "sensors/#"
    /// 
    /// Örnek: `MQTT_TOPICS=sensors/#,devices/+/status`
    #[serde(default = "default_topics")]
    pub mqtt_topics: String,

    /// Logging seviyesi
    /// 
    /// Geçerli değerler: error, warn, info, debug, trace
    /// 
    /// Varsayılan: "info"
    /// 
    /// Örnek: `RUST_LOG=debug`
    #[serde(default = "default_log")]
    pub log_level: String,
}

// Varsayılan değer fonksiyonları
fn default_broker_host() -> String { "localhost".into() }
fn default_broker_port() -> u16 { 1883 }
fn default_client_id() -> String { "rustyflow-gateway".into() }
fn default_topics() -> String { "sensors/#".into() }
fn default_log() -> String { "info".into() }

impl Config {
    /// Yapılandırmayı yükle
    /// 
    /// Yükleme sırası:
    /// 1. .env dosyasını oku
    /// 2. Ortam değişkenlerini parse et
    /// 3. Varsayılan değerleri kullan
    /// 
    /// # Örnek
    /// ```ignore
    /// let config = Config::load();
    /// println!("MQTT Broker: {}:{}", config.mqtt_broker_host, config.mqtt_broker_port);
    /// ```
    pub fn load() -> Self {
        // .env dosyasını yükle (hata görmezden gel)
        let _ = dotenvy::dotenv();

        // Ortam değişkenlerini Config struct'ına dönüştür
        let mut cfg: Config = envy::from_env().unwrap_or_else(|_| Config {
            mqtt_broker_host: default_broker_host(),
            mqtt_broker_port: default_broker_port(),
            mqtt_client_id: default_client_id(),
            mqtt_topics: default_topics(),
            log_level: default_log(),
        });

        // RUST_LOG özel işlemi
        if let Ok(level) = std::env::var("RUST_LOG") {
            cfg.log_level = level;
        }

        cfg
    }

    /// Topic listesini parse et (virgülle ayrılmış → Vec<String>)
    /// 
    /// # Örnek
    /// ```ignore
    /// let config = Config::load();
    /// let topics = config.parse_topics(); // ["sensors/#", "devices/+/status"]
    /// ```
    pub fn parse_topics(&self) -> Vec<String> {
        self.mqtt_topics
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}
