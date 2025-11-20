//! Mock Sensörler
//!
//! Gerçek sensörleri simüle eden mock veri üreteçleri.
//! Raspberry Pi'da gerçek sensörler olsaydı, bunların yerine
//! rppal veya embedded-hal kullanarak gerçek okumalar yapılırdı.

use rand::Rng;
use shared_types::sensor::SensorReading;
use chrono::Utc;
use uuid::Uuid;

/// Sensör okuması ve tip bilgisi
#[derive(Debug, Clone)]
pub struct SensorData {
    pub reading: SensorReading,
    pub sensor_type: String,
    pub unit: String,
}

/// Sıcaklık sensörü (mock)
/// 
/// 18-30°C arasında rastgele değerler üretir.
/// Gerçek kullanımda: DHT22, DS18B20 vb. sensörlerden okuma yapılır.
pub struct TemperatureSensor {
    sensor_id: Uuid,
    last_value: f64,
}

impl TemperatureSensor {
    /// Yeni sıcaklık sensörü oluştur
    pub fn new() -> Self {
        Self {
            sensor_id: Uuid::new_v4(),
            last_value: 22.0, // Başlangıç değeri (oda sıcaklığı)
        }
    }

    /// Mock sıcaklık verisi üret
    /// 
    /// Gerçekçi olması için son değere yakın bir değer üretir (±2°C)
    pub fn read(&mut self) -> SensorData {
        let mut rng = rand::thread_rng();
        
        // Son değere göre küçük değişiklik yap (daha gerçekçi)
        let change: f64 = rng.gen_range(-2.0..2.0);
        self.last_value = (self.last_value + change).clamp(18.0, 30.0);

        SensorData {
            reading: SensorReading {
                sensor_id: self.sensor_id,
                value: format!("{:.2}", self.last_value),
                timestamp: Utc::now(),
                is_valid: true,
                metadata: None,
            },
            sensor_type: "temperature".to_string(),
            unit: "celsius".to_string(),
        }
    }
}

/// Nem sensörü (mock)
/// 
/// 30-80% arasında rastgele nem değerleri üretir.
/// Gerçek kullanımda: DHT22, BME280 vb. sensörlerden okuma yapılır.
pub struct HumiditySensor {
    sensor_id: Uuid,
    last_value: f64,
}

impl HumiditySensor {
    /// Yeni nem sensörü oluştur
    pub fn new() -> Self {
        Self {
            sensor_id: Uuid::new_v4(),
            last_value: 55.0, // Başlangıç değeri (orta nem)
        }
    }

    /// Mock nem verisi üret
    pub fn read(&mut self) -> SensorData {
        let mut rng = rand::thread_rng();
        
        // Son değere göre küçük değişiklik yap
        let change: f64 = rng.gen_range(-5.0..5.0);
        self.last_value = (self.last_value + change).clamp(30.0, 80.0);

        SensorData {
            reading: SensorReading {
                sensor_id: self.sensor_id,
                value: format!("{:.1}", self.last_value),
                timestamp: Utc::now(),
                is_valid: true,
                metadata: None,
            },
            sensor_type: "humidity".to_string(),
            unit: "percent".to_string(),
        }
    }
}

/// Hareket sensörü (mock)
/// 
/// %20 olasılıkla hareket algılar (1.0), yoksa 0.0 döner.
/// Gerçek kullanımda: PIR sensör (HC-SR501) ile gerçek hareket algılama.
pub struct MotionSensor {
    sensor_id: Uuid,
}

impl MotionSensor {
    /// Yeni hareket sensörü oluştur
    pub fn new() -> Self {
        Self {
            sensor_id: Uuid::new_v4(),
        }
    }

    /// Mock hareket verisi üret
    /// 
    /// "1" = Hareket algılandı
    /// "0" = Hareket yok
    pub fn read(&self) -> SensorData {
        let mut rng = rand::thread_rng();
        let motion_detected = rng.gen_bool(0.2); // %20 olasılık

        SensorData {
            reading: SensorReading {
                sensor_id: self.sensor_id,
                value: if motion_detected { "1".to_string() } else { "0".to_string() },
                timestamp: Utc::now(),
                is_valid: true,
                metadata: if motion_detected {
                    Some(serde_json::json!({"event": "motion_detected"}))
                } else {
                    None
                },
            },
            sensor_type: "motion".to_string(),
            unit: "boolean".to_string(),
        }
    }
}

/// Tüm sensörleri yöneten controller
/// 
/// Gerçek Raspberry Pi'da GPIO pinlerine bağlı sensörleri yönetir.
pub struct SensorController {
    pub temperature: TemperatureSensor,
    pub humidity: HumiditySensor,
    pub motion: MotionSensor,
}

impl SensorController {
    /// Yeni sensör controller oluştur
    /// 
    /// Gerçek kullanımda: GPIO pinlerini initialize eder
    pub fn new() -> Self {
        Self {
            temperature: TemperatureSensor::new(),
            humidity: HumiditySensor::new(),
            motion: MotionSensor::new(),
        }
    }

    /// Tüm sensörlerden veri oku
    /// 
    /// Her sensörden bir okuma yapar ve SensorData vector'ü döner.
    pub fn read_all(&mut self) -> Vec<SensorData> {
        vec![
            self.temperature.read(),
            self.humidity.read(),
            self.motion.read(),
        ]
    }
}
