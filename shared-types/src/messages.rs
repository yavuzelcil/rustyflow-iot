//! MQTT Message Types
//!
//! MQTT gateway ve edge agents arasında iletişim için kullanılan message tipler.
//! JSON formatında serializasyon destekler.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// MQTT üzerinden gönderilen genel mesaj
/// 
/// MQTT topic'lerine publish edilen mesajların yapısı.
/// 
/// # Örnekler
/// 
/// - Sensör verisi: `/devices/rpi-01/sensors/temp`
/// - Device kontrol: `/devices/rpi-01/commands/led-on`
/// - Durum güncelleme: `/devices/rpi-01/status`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttMessage {
    /// Mesajın türü
    pub message_type: String,
    
    /// Mesajın içeriği (JSON)
    pub payload: serde_json::Value,
    
    /// Mesajın gönderildiği zaman
    pub timestamp: DateTime<Utc>,
    
    /// Mesajı gönderen cihazın ID'si
    pub device_id: Uuid,
    
    /// MQTT QoS seviyesi (0, 1, veya 2)
    #[serde(default)]
    pub qos: u8,
}

/// Edge agent'lar tarafından gönderilen device mesajı
/// 
/// Sensör verileri, durum güncellemeleri, hata raporları vb.
/// 
/// # Örnek JSON (Sensör Verisi)
/// ```json
/// {
///   "device_id": "550e8400-e29b-41d4-a716-446655440000",
///   "command": "sensor_reading",
///   "data": {
///     "sensor_id": "550e8400-e29b-41d4-a716-446655440001",
///     "value": "23.5",
///     "type": "temperature"
///   },
///   "timestamp": "2024-11-13T21:30:00Z"
/// }
/// ```
/// 
/// # Örnek JSON (Durum Güncellemesi)
/// ```json
/// {
///   "device_id": "550e8400-e29b-41d4-a716-446655440000",
///   "command": "status_update",
///   "data": {
///     "uptime": 3600,
///     "cpu_temp": 45.2,
///     "memory_free": 512
///   },
///   "timestamp": "2024-11-13T21:30:00Z"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMessage {
    /// Mesajı gönderen cihazın ID'si
    pub device_id: Uuid,
    
    /// Komut veya mesaj türü
    /// Örnekler: "sensor_reading", "status_update", "error_report", "heartbeat"
    pub command: String,
    
    /// Mesajın içeriği (JSON, yapı flexible)
    pub data: serde_json::Value,
    
    /// Mesajın oluşturulduğu zaman
    pub timestamp: DateTime<Utc>,
}

/// API server'dan edge agent'a gönderilen komut
/// 
/// LED kontrolü, kamera çekim komutu, sensör kalibrasyonu vb.
/// 
/// # Örnek JSON (LED Kontrolü)
/// ```json
/// {
///   "device_id": "550e8400-e29b-41d4-a716-446655440000",
///   "command_type": "control",
///   "command_name": "led_on",
///   "parameters": {
///     "led_id": "led_01",
///     "brightness": 255
///   },
///   "correlation_id": "550e8400-e29b-41d4-a716-446655440003",
///   "timestamp": "2024-11-13T21:30:00Z"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCommand {
    /// Komutun gönderileceği cihaz
    pub device_id: Uuid,
    
    /// Komut kategorisi: "control", "config", "maintenance"
    pub command_type: String,
    
    /// Komut adı (örn: "led_on", "take_photo", "calibrate")
    pub command_name: String,
    
    /// Komut parametreleri (flexible JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    
    /// Yanıtları correlation için ID (request-response matching)
    pub correlation_id: Uuid,
    
    /// Komutun gönderildiği zaman
    pub timestamp: DateTime<Utc>,
}

impl MqttMessage {
    /// Yeni bir MQTT mesajı oluştur
    pub fn new(
        message_type: String,
        payload: serde_json::Value,
        device_id: Uuid,
    ) -> Self {
        Self {
            message_type,
            payload,
            timestamp: Utc::now(),
            device_id,
            qos: 1, // Default: At-least-once delivery
        }
    }

    /// QoS değerini ayarla
    pub fn with_qos(mut self, qos: u8) -> Self {
        self.qos = qos.min(2); // Max QoS: 2
        self
    }
}

impl DeviceMessage {
    /// Yeni bir device mesajı oluştur
    pub fn new(
        device_id: Uuid,
        command: String,
        data: serde_json::Value,
    ) -> Self {
        Self {
            device_id,
            command,
            data,
            timestamp: Utc::now(),
        }
    }
}

impl DeviceCommand {
    /// Yeni bir device komutu oluştur
    pub fn new(
        device_id: Uuid,
        command_type: String,
        command_name: String,
    ) -> Self {
        Self {
            device_id,
            command_type,
            command_name,
            parameters: None,
            correlation_id: Uuid::new_v4(),
            timestamp: Utc::now(),
        }
    }

    /// Komuta parametreler ekle
    pub fn with_parameters(mut self, parameters: serde_json::Value) -> Self {
        self.parameters = Some(parameters);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mqtt_message() {
        let device_id = Uuid::new_v4();
        let payload = serde_json::json!({"temperature": 23.5});
        let msg = MqttMessage::new("sensor_data".to_string(), payload, device_id);
        
        assert_eq!(msg.message_type, "sensor_data");
        assert_eq!(msg.device_id, device_id);
        assert_eq!(msg.qos, 1);
    }

    #[test]
    fn test_device_message() {
        let device_id = Uuid::new_v4();
        let data = serde_json::json!({"value": "on"});
        let msg = DeviceMessage::new(device_id, "status".to_string(), data);
        
        assert_eq!(msg.command, "status");
        assert_eq!(msg.device_id, device_id);
    }

    #[test]
    fn test_device_command() {
        let device_id = Uuid::new_v4();
        let cmd = DeviceCommand::new(
            device_id,
            "control".to_string(),
            "led_on".to_string(),
        );
        
        assert_eq!(cmd.command_name, "led_on");
        assert_eq!(cmd.device_id, device_id);
    }
}
