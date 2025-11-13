//! RustyFlow IoT Platform - API Server
//!
//! Bu modül, RustyFlow IoT platformunun REST API sunucusunu oluşturur.
//! - PostgreSQL veritabanı ile media yönetimi
//! - Yapılandırma sistemi (.env dosyasından)
//! - Sağlık kontrol endpointleri
//! - Graceful shutdown desteği

mod routes;      // HTTP endpoint handler'ları
mod config;      // Konfigürasyon sistemi
mod state;       // Uygulama durumu ve shared state

use axum::{Router, routing::{get, post, put, delete}};
use tracing_subscriber;
use config::Config;
use state::AppState;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use sqlx::postgres::PgPoolOptions;

/// Tokio async runtime ile ana uygulama giriş noktası
#[tokio::main]
async fn main() {
    // ========== 1. KONFIGURASYON ==========
    // .env dosyasından ortam değişkenlerini oku
    // Fallback değerleri: APP_PORT=3000, RUST_LOG=info
    let cfg = Config::load();

    // ========== 2. LOGGING SISTEMI ==========
    // Structured logging'i başlat
    // Log seviyesi ortam değişkeninden okunur (info, debug, vb.)
    tracing_subscriber::fmt()
        .with_env_filter(cfg.log_level.clone())
        .init();

    // ========== 3. IN-MEMORY STORE ==========
    // Media verilerini geçici olarak saklamak için (fallback amaçlı)
    // Arc<RwLock<>> = thread-safe, async-compatible shared state
    let store = Arc::new(RwLock::new(HashMap::new()));

    // ========== 4. DATABASE BAĞLANTISI ==========
    // PostgreSQL connection pool'u oluştur
    // DATABASE_URL ortam değişkeni varsa: "postgres://user:pass@host:port/db"
    // Yoksa fallback olarak in-memory store kullan
    let db_pool = if let Some(url) = cfg.database_url.clone() {
        match PgPoolOptions::new()
            .max_connections(5)                    // Maksimum 5 eş zamanlı bağlantı
            .acquire_timeout(std::time::Duration::from_secs(2))  // Timeout: 2 saniye
            .connect(&url)
            .await
        {
            Ok(pool) => {
                tracing::info!("DB connected");
                Some(pool)
            }
            Err(e) => {
                tracing::warn!("DB connection failed: {e}");
                None
            }
        }
    } else {
        None
    };

    // ========== 5. APPLICATION STATE ==========
    // Tüm handler'lara pass edilecek shared state
    // - cfg: konfigürasyon
    // - media_store: in-memory fallback
    // - db: PostgreSQL pool (optional)
    let app_state = AppState { cfg: cfg.clone(), media_store: store, db: db_pool };

    // ========== 6. HTTP ROUTER ==========
    // Axum router ile tüm endpoint'leri tanımla
    let app = Router::new()
        // Sistem ve sağlık kontrol endpoint'leri
        .route("/",           get(routes::health::root))      // Status check
        .route("/health",     get(routes::health::health))    // Sağlık durumu
        .route("/ready",      get(routes::health::ready))     // Hazır mı?
        .route("/v1/config",  get(routes::sys::config))       // Yapılandırma
        // Media CRUD endpoint'leri (v1 API)
        .route("/v1/media",         post(routes::media::create_media).get(routes::media::list_media))
        .route("/v1/media/{id}",    get(routes::media::get_media))
        .route("/v1/media/{id}",    put(routes::media::update_media))
        .route("/v1/media/{id}",    delete(routes::media::delete_media))
        // Database sağlık kontrol
        .route("/db/health", get(|| async { "ok" }))
        // Shared state'i tüm handler'lara inject et
        .with_state(app_state);

    // ========== 7. SERVER BAŞLAT ==========
    // Sunucu adresi: 0.0.0.0:3000 (tüm interfaces'den dinle)
    let addr = std::net::SocketAddr::from(([0,0,0,0], cfg.app_port));
    tracing::info!("api-server listening on http://{addr}");

    // ========== 8. GRACEFUL SHUTDOWN ==========
    // Graceful shutdown ile sunucuyu başlat
    // CTRL+C sinyali gelince nazikçe kapat
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

/// CTRL+C (SIGINT) sinyalini dinle ve shutdown'u tetikle
/// 
/// Bu fonksiyon, sunucu işlemlerini sorunsuz bir şekilde sonlandırmak için
/// kullanıcının CTRL+C tuşlamasını bekler.
async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received, exiting...");
}
