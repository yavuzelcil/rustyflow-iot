/// Sensör endpoint'leri
/// 
/// MQTT gateway'den gelen sensör verilerini Redis'te cache'leyip web dashboard'a sunar.
/// Redis bağlantısı yoksa in-memory HashMap fallback kullanır.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use redis::AsyncCommands;
use crate::state::AppState;

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

/// Redis key prefix - Tüm sensor key'leri bu prefix ile başlar
const REDIS_KEY_PREFIX: &str = "sensor:";

/// Tüm sensör verilerini listele
/// 
/// GET /api/sensors
/// 
/// Redis'ten tüm sensor:* key'lerini okur ve JSON array döner.
/// Redis bağlantısı yoksa boş array döner.
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
    State(state): State<AppState>,
) -> Result<Json<Vec<SensorData>>, StatusCode> {
    // Redis varsa Redis'ten oku
    if let Some(mut redis_conn) = state.redis.clone() {
        match get_all_sensors_from_redis(&mut redis_conn).await {
            Ok(sensors) => return Ok(Json(sensors)),
            Err(e) => {
                tracing::warn!("Redis read error: {e}, returning empty list");
                return Ok(Json(vec![]));
            }
        }
    }
    
    // Redis yoksa boş liste dön
    tracing::debug!("Redis not available, returning empty sensor list");
    Ok(Json(vec![]))
}

/// Redis'ten tüm sensör verilerini oku
async fn get_all_sensors_from_redis(
    conn: &mut redis::aio::ConnectionManager,
) -> Result<Vec<SensorData>, Box<dyn std::error::Error>> {
    // sensor:* pattern'ine uyan tüm key'leri bul
    let keys: Vec<String> = conn.keys(format!("{}*", REDIS_KEY_PREFIX)).await?;
    
    let mut sensors = Vec::new();
    
    // Her key için değeri oku
    for key in keys {
        let json: String = conn.get(&key).await?;
        if let Ok(sensor) = serde_json::from_str::<SensorData>(&json) {
            sensors.push(sensor);
        }
    }
    
    Ok(sensors)
}

/// Yeni sensör verisi ekle (MQTT gateway tarafından kullanılır)
/// 
/// POST /api/sensors
/// 
/// Redis'e JSON olarak yazar.
/// Key format: "sensor:device_id:sensor_type"
/// Value: JSON serialized SensorData
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
    State(state): State<AppState>,
    Json(data): Json<SensorData>,
) -> Result<StatusCode, StatusCode> {
    // Redis varsa Redis'e yaz
    if let Some(mut redis_conn) = state.redis.clone() {
        let key = format!("{}{}:{}", REDIS_KEY_PREFIX, data.device_id, data.sensor_type);
        
        match serde_json::to_string(&data) {
            Ok(json) => {
                // Redis'e JSON string olarak kaydet
                // TTL 1 saat (3600 saniye) - eski veriler otomatik silinir
                match redis_conn.set_ex::<_, _, ()>(&key, json, 3600).await {
                    Ok(_) => {
                        tracing::debug!("Sensor data saved to Redis: {key}");
                        return Ok(StatusCode::OK);
                    }
                    Err(e) => {
                        tracing::error!("Redis write error: {e}");
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
            }
            Err(e) => {
                tracing::error!("JSON serialization error: {e}");
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }
    
    // Redis yoksa hata dön
    tracing::warn!("Redis not available, sensor data not saved");
    Err(StatusCode::SERVICE_UNAVAILABLE)
}
