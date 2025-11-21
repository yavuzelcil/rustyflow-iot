/// API client modülü
/// 
/// Bu modül API server'dan veri çekmek için kullanılır.
/// gloo-net ile HTTP request'leri yapar.

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

/// Sensör verisi - API'den gelen format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub device_id: String,
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
    pub metadata: Option<serde_json::Value>,
}

/// API'den sensör verilerini çeker
/// 
/// Şu an için mock data döndürüyor çünkü henüz API endpoint'imiz yok.
/// Gelecekte gerçek API endpoint'e bağlanacak.
pub async fn fetch_sensor_data() -> Result<Vec<SensorData>, String> {
    // API URL'i - development için localhost
    let api_url = "http://localhost:3000/api/sensors";
    
    // API'ye request at
    let response = Request::get(api_url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch data: {}", e))?;
    
    // Eğer API cevap vermediyse, mock data dön
    if !response.ok() {
        // Mock data - gerçek sensör verileri gibi görünsün
        return Ok(vec![
            SensorData {
                device_id: "edge-agent-001".to_string(),
                sensor_type: "temperature".to_string(),
                value: 23.5,
                unit: "°C".to_string(),
                timestamp: "2024-01-20T10:30:00Z".to_string(),
                metadata: None,
            },
            SensorData {
                device_id: "edge-agent-001".to_string(),
                sensor_type: "humidity".to_string(),
                value: 58.2,
                unit: "%".to_string(),
                timestamp: "2024-01-20T10:30:00Z".to_string(),
                metadata: None,
            },
            SensorData {
                device_id: "edge-agent-001".to_string(),
                sensor_type: "motion".to_string(),
                value: 1.0,
                unit: "bool".to_string(),
                timestamp: "2024-01-20T10:30:00Z".to_string(),
                metadata: Some(serde_json::json!({"event": "motion_detected"})),
            },
        ]);
    }
    
    // JSON response'u parse et
    response
        .json::<Vec<SensorData>>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}
