//! Uygulama Durumu (Application State)
//!
//! Tüm HTTP handler'larına inject edilen shared state.
//! Thread-safe ve async-compatible veri yapıları içerir.

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use shared_types::Media;

use crate::config::Config;

/// Uygulama global durumu
/// 
/// Axum'un `with_state()` metodu ile tüm route handler'larına inject edilir.
/// 
/// Bu struct, request'ler arasında shared ve mutable olan verileri içerir.
/// Thread-safe olması için `Arc` ve `RwLock` kullanılır.
/// 
/// # Bileşenler
/// 
/// - **cfg**: Sunucu konfigürasyonu (port, database URL, log level)
/// - **media_store**: In-memory fallback storage (PostgreSQL yoksa kullan)
/// - **db**: PostgreSQL connection pool (optional)
/// 
/// # Örnek Kullanım
/// 
/// Handler'da AppState'i almak:
/// ```ignore
/// pub async fn handler(
///     State(state): State<AppState>
/// ) -> String {
///     // state.cfg, state.db, state.media_store'a erişebilirsin
///     state.cfg.app_port.to_string()
/// }
/// ```
#[derive(Clone)]
pub struct AppState {
    /// Sunucu yapılandırması
    /// 
    /// Port, database URL, log level gibi ayarları içerir
    pub cfg: Config,
    
    /// In-memory media storage (fallback amaçlı)
    /// 
    /// PostgreSQL bağlanmazsa, media verileri burada saklanır.
    /// 
    /// Detay:
    /// - `Arc<...>` = Atomic Reference Count, thread-safe shared ownership
    /// - `RwLock<...>` = Async-compatible okuma/yazma lock
    ///   - Çoklu eş zamanlı okumaları destekler
    ///   - Yazma için exclusive lock alır
    /// - `HashMap<Uuid, Media>` = ID -> Media eşlemesi
    pub media_store: Arc<RwLock<HashMap<Uuid, Media>>>,

    /// PostgreSQL connection pool
    /// 
    /// `Option<PgPool>` çünkü veritabanı bağlanması başarısız olabilir.
    /// 
    /// SQLx pool otomatik olarak:
    /// - Connection'ları reuse eder (performans)
    /// - Timeout'ları yönetir
    /// - Health check'ler yapar
    pub db: Option<PgPool>,

    /// Redis connection manager
    /// 
    /// Sensor cache için kullanılır (in-memory HashMap yerine).
    /// 
    /// `Option<ConnectionManager>` çünkü Redis bağlanması başarısız olabilir.
    /// 
    /// ConnectionManager otomatik olarak:
    /// - Connection'ı reuse eder (performans)
    /// - Kopan bağlantıyı yeniden kurar (auto-reconnect)
    /// - Timeout'ları yönetir
    /// - Async-compatible (tokio ile çalışır)
    pub redis: Option<ConnectionManager>,
}