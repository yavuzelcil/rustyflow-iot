/// Sensör kartı component'i
/// 
/// Her bir sensör için ayrı bir kart gösterir.
/// Sıcaklık, nem, hareket gibi farklı sensör tiplerini destekler.

use leptos::*;
use crate::api::SensorData;

#[component]
pub fn SensorCard(sensor: SensorData) -> impl IntoView {
    // Sensör tipine göre CSS class
    let sensor_class = format!("sensor-card {}", sensor.sensor_type);
    
    // Sensör değerini formatla
    let formatted_value = if sensor.sensor_type == "motion" {
        if sensor.value > 0.0 {
            "DETECTED".to_string()
        } else {
            "IDLE".to_string()
        }
    } else {
        format!("{:.1}", sensor.value)
    };
    
    // Sensör ismini formatla (ilk harfi büyük)
    let sensor_name = sensor.sensor_type
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_uppercase().to_string() } else { c.to_string() })
        .collect::<String>();
    
    // Timestamp'i formatla (sadece saat:dakika)
    let formatted_time = sensor.timestamp
        .split('T')
        .nth(1)
        .and_then(|t| Some(t.split(':').take(2).collect::<Vec<_>>().join(":")))
        .unwrap_or_else(|| "N/A".to_string());
    
    // Device ID'nin son kısmını al (static string için)
    let device_short = sensor.device_id
        .split('-')
        .last()
        .unwrap_or("N/A")
        .to_string();

    view! {
        <div class=sensor_class>
            <div class="sensor-header">
                <div class="sensor-name">{sensor_name}</div>
                <div class="sensor-status status-online">"Online"</div>
            </div>

            {
                if sensor.sensor_type == "motion" {
                    // Motion sensor için özel görünüm
                    let motion_class = if sensor.value > 0.0 {
                        "motion-indicator motion-detected"
                    } else {
                        "motion-indicator motion-idle"
                    };
                    
                    view! {
                        <div>
                            <div class=motion_class>
                                {if sensor.value > 0.0 { "🚶" } else { "💤" }}
                            </div>
                            <div style="text-align: center; font-size: 1.5rem; font-weight: 600; color: #333;">
                                {formatted_value}
                            </div>
                        </div>
                    }.into_view()
                } else {
                    // Diğer sensörler için normal görünüm
                    view! {
                        <div class="sensor-value">
                            {formatted_value}
                            <span class="sensor-unit">{&sensor.unit}</span>
                        </div>
                    }.into_view()
                }
            }

            <div class="sensor-info">
                <div class="info-item">
                    <div class="info-label">"Device"</div>
                    <div class="info-value">{device_short}</div>
                </div>
                <div class="info-item">
                    <div class="info-label">"Last Update"</div>
                    <div class="info-value">{formatted_time}</div>
                </div>
            </div>
        </div>
    }
}
