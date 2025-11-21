/// Sensör endpoint'leri
/// 
/// MQTT gateway'den gelen sensör verilerini cache'leyip web dashboard'a sunar.
/// Şimdilik in-memory cache kullanıyor, gelecekte Redis gibi bir cache kullanılabilir.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// Sensör verisi - Dashboard'a gönderilen format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub device_id: String,
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
    pub metadata: Option<serde_json::Value>,
}

/// In-memory sensör cache'i
/// 
/// Key: "device_id:sensor_type" formatında (örn: "edge-agent-001:temperature")
/// Value: En son sensör verisi
pub type SensorCache = Arc<RwLock<HashMap<String, SensorData>>>;

/// Tüm sensör verilerini listele
/// 
/// GET /api/sensors
/// 
/// Response:
/// ```json
/// [
///   {
///     "device_id": "edge-agent-001",
///     "sensor_type": "temperature",
///     "value": 23.5,
///     "unit": "°C",
///     "timestamp": "2024-01-20T10:30:00Z",
///     "metadata": null
///   }
/// ]
/// ```
pub async fn list_sensors(
    State(cache): State<SensorCache>,
) -> Result<Json<Vec<SensorData>>, StatusCode> {
    let cache = cache.read().await;
    let sensors: Vec<SensorData> = cache.values().cloned().collect();
    Ok(Json(sensors))
}

/// Yeni sensör verisi ekle (MQTT gateway tarafından kullanılır)
/// 
/// POST /api/sensors
/// 
/// Body:
/// ```json
/// {
///   "device_id": "edge-agent-001",
///   "sensor_type": "temperature",
///   "value": 23.5,
///   "unit": "°C",
///   "timestamp": "2024-01-20T10:30:00Z"
/// }
/// ```
pub async fn add_sensor_data(
    State(cache): State<SensorCache>,
    Json(data): Json<SensorData>,
) -> Result<StatusCode, StatusCode> {
    let key = format!("{}:{}", data.device_id, data.sensor_type);
    
    let mut cache = cache.write().await;
    cache.insert(key, data);
    
    Ok(StatusCode::OK)
}
