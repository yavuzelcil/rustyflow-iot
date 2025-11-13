//! Sağlık Kontrol Endpoint'leri (Health Check Routes)
//!
//! Sunucunun ve servislerinin sağlık durumunu kontrol etmek için kullanılan endpoint'ler.
//! Kubernetes ve diğer orchestration araçları tarafından kullanılır.

use axum::{response::IntoResponse, Json};
use serde::Serialize;

/// Basit sağlık durumu response'ı
#[derive(Serialize)]
struct Health { 
    /// Durum: "ok" veya "ready"
    status: &'static str 
}

/// Root endpoint - Basit status check
/// 
/// # HTTP
/// `GET /`
/// 
/// # Response
/// ```text
/// "RustyFlow API — OK"
/// ```
/// 
/// # Amaç
/// Basit liveness check için. Sunucunun çalışıp çalışmadığını hızlı biçimde kontrol et.
pub async fn root() -> &'static str {
    "RustyFlow API — OK"
}

/// Health endpoint - Sunucunun sağlığını kontrol et
/// 
/// # HTTP
/// `GET /health`
/// 
/// # Response
/// ```json
/// {"status":"ok"}
/// ```
/// 
/// # Amaç
/// Liveness probe için. Konteyner orchestration araçları (Kubernetes, Docker Compose)
/// sunucunun canlı olup olmadığını kontrol eder.
pub async fn health() -> impl IntoResponse {
    Json(Health { status: "ok" })
}

/// Ready endpoint - Sunucu hazır mı?
/// 
/// # HTTP
/// `GET /ready`
/// 
/// # Response
/// ```json
/// {"status":"ready"}
/// ```
/// 
/// # Amaç
/// Readiness probe için. Sunucunun istek kabul etmeye hazır olup olmadığını kontrol et.
/// 
/// **Not**: İleride database bağlantısı, message queue, cache vb. kontroller eklenebilir.
pub async fn ready() -> impl IntoResponse {
    // ileride: DB bağlantısı, mqtt bağlı mı gibi kontroller.
    Json(Health { status: "ready" })
}