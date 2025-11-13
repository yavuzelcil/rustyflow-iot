//! Sistem Endpoint'leri (System Routes)
//!
//! Sunucunun ve uygulamanın yapılandırması hakkında bilgi sağlayan endpoint'ler.

use axum::{extract::State, response::IntoResponse, Json};
use crate::state::AppState;

/// Sunucu yapılandırmasını döndür
/// 
/// # HTTP
/// `GET /v1/config`
/// 
/// # Response
/// ```json
/// {
///   "app_port": 3000,
///   "has_database_url": true,
///   "log_level": "info"
/// }
/// ```
/// 
/// # Amaç
/// İstemci ve monitoring araçlarının sunucunun yapılandırmasını öğrenmesi için.
/// 
/// **Güvenlik Notu**: Hassas bilgiler (database şifresi vb.) maskelenmiştir.
/// Sadece "var mı yok mu" bilgisi döndürülür.
pub async fn config(State(st): State<AppState>) -> impl IntoResponse {
    // st.cfg.sanitized() = Güvenli yapılandırma
    // Veritabanı URL'sinin full değeri yerine has_database_url: true/false döndür
    Json(st.cfg.sanitized())
}