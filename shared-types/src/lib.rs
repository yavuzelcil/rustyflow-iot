//! RustyFlow Shared Types Library
//!
//! Tüm RustyFlow IoT Platform mikroservislerinde kullanılan ortak veri yapılarını içerir.
//! 
//! Bu kütüphane aşağıdaki servislerde kullanılır:
//! - API Server (Axum)
//! - MQTT Gateway
//! - Edge Agent (Raspberry Pi)
//! - ML Service
//! 
//! # Amaç
//! 
//! Tüm projelerde aynı data type'ları kullanarak:
//! - Kod tekrarını (code duplication) elimine et
//! - Veri uyumluluğu sağla (data consistency)
//! - Serializasyon sorunlarını önle
//! - Maintenance'i kolaylaştır

pub mod media;
pub mod error;
pub mod sensor;
pub mod messages;

// Re-export sık kullanılan tipler
pub use media::{Media, NewMedia, UpdateMedia};
pub use error::{Result, Error};
pub use sensor::{Sensor, SensorReading};
pub use messages::{MqttMessage, DeviceMessage};
