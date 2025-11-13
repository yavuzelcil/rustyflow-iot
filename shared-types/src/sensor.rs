//! Sensor Types
//!
//! Sensör tanımları ve sensörden gelen verileri temsil eden tipler.
//! Edge agent'lar ve IoT cihazları bu tipler üzerinden veri gönderir.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Sensör cihazının tanımlanması
/// 
/// Bir Raspberry Pi'daki veya başka bir edge device'daki sensör.
/// Sıcaklık, nem, basınç, hareket vb. sensörleri temsil edebilir.
/// 
/// # Alanlar
/// 
/// - `id`: UUID benzersiz tanımlayıcı
/// - `device_id`: Sensörün bağlı olduğu cihazın ID'si
/// - `name`: Sensörün adı (örn: "room-temperature-sensor")
/// - `sensor_type`: Sensör tipi (örn: "temperature", "humidity", "motion")
/// - `unit`: Ölçüm birimi (örn: "°C", "%", "m/s²")
/// - `location`: Sensörün fiziksel konumu (örn: "bedroom", "kitchen")
/// 
/// # Örnek JSON
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440001",
///   "device_id": "550e8400-e29b-41d4-a716-446655440000",
///   "name": "room-temperature",
///   "sensor_type": "temperature",
///   "unit": "°C",
///   "location": "bedroom"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    pub id: Uuid,
    pub device_id: Uuid,
    pub name: String,
    pub sensor_type: String,
    pub unit: String,
    pub location: String,
}

/// Sensörden gelen tek bir veri okuma (reading)
/// 
/// Sensörün belirli bir andaki ölçümünü temsil eder.
/// Edge agent'lar bu veriyi MQTT üzerinden gönderir.
/// 
/// # Alanlar
/// 
/// - `sensor_id`: Hangi sensörden geldiği
/// - `value`: Ölçüm değeri (float olabilir veya string)
/// - `timestamp`: Ölçümün alındığı zaman (ISO 8601)
/// - `is_valid`: Veri geçerli mi? (hatalı okumalar işaretlenebilir)
/// - `metadata`: Ek bilgiler (opsiyonel)
/// 
/// # Örnek JSON (Sıcaklık)
/// ```json
/// {
///   "sensor_id": "550e8400-e29b-41d4-a716-446655440001",
///   "value": "23.5",
///   "timestamp": "2024-11-13T21:30:00Z",
///   "is_valid": true,
///   "metadata": null
/// }
/// ```
/// 
/// # Örnek JSON (Hareket Sensörü)
/// ```json
/// {
///   "sensor_id": "550e8400-e29b-41d4-a716-446655440002",
///   "value": "motion_detected",
///   "timestamp": "2024-11-13T21:30:15Z",
///   "is_valid": true,
///   "metadata": {"duration_ms": 500}
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_id: Uuid,
    pub value: String,
    pub timestamp: DateTime<Utc>,
    pub is_valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Sensor {
    /// Yeni bir Sensor oluştur
    pub fn new(
        device_id: Uuid,
        name: String,
        sensor_type: String,
        unit: String,
        location: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            device_id,
            name,
            sensor_type,
            unit,
            location,
        }
    }
}

impl SensorReading {
    /// Yeni bir SensorReading oluştur
    pub fn new(sensor_id: Uuid, value: String) -> Self {
        Self {
            sensor_id,
            value,
            timestamp: Utc::now(),
            is_valid: true,
            metadata: None,
        }
    }

    /// SensorReading'e metadata ekle
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Reading'i invalid işaretle
    pub fn mark_invalid(mut self) -> Self {
        self.is_valid = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sensor() {
        let device_id = Uuid::new_v4();
        let sensor = Sensor::new(
            device_id,
            "temp-sensor".to_string(),
            "temperature".to_string(),
            "°C".to_string(),
            "bedroom".to_string(),
        );
        
        assert_eq!(sensor.name, "temp-sensor");
        assert_eq!(sensor.device_id, device_id);
    }

    #[test]
    fn test_sensor_reading() {
        let sensor_id = Uuid::new_v4();
        let reading = SensorReading::new(sensor_id, "23.5".to_string());
        
        assert_eq!(reading.sensor_id, sensor_id);
        assert_eq!(reading.value, "23.5");
        assert!(reading.is_valid);
    }
}
