mod routes;
mod config;
mod state;

use axum::{Router, routing::{get, post, put, delete}};
use tracing_subscriber;
use config::Config;
use state::AppState;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // Config yükle
    let cfg = Config::load();

    // logging
    tracing_subscriber::fmt()
        .with_env_filter(cfg.log_level.clone())
        .init();

    // Boş in-memory store
    let store = Arc::new(RwLock::new(HashMap::new()));

    let db_pool = if let Some(url) = cfg.database_url.clone() {
        match PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(2))
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


    let app_state = AppState { cfg: cfg.clone(), media_store: store, db: db_pool };

    // Router
    let app = Router::new()
        // sistem/health
        .route("/",           get(routes::health::root))
        .route("/health",     get(routes::health::health))
        .route("/ready",      get(routes::health::ready))
        .route("/v1/config",  get(routes::sys::config))
        // media CRUD (in-memory)
        .route("/v1/media",           post(routes::media::create_media).get(routes::media::list_media))
        .route("/v1/media/{id}",       get(routes::media::get_media))
        .route("/v1/media/{id}",       put(routes::media::update_media))
        .route("/v1/media/{id}",       delete(routes::media::delete_media))
        // AppState'i tüm handler'lara ver
        .route("/db/health", get(|| async { "ok" }))
        .with_state(app_state);

    // Portu config'ten al
    let addr = std::net::SocketAddr::from(([0,0,0,0], cfg.app_port));
    tracing::info!("api-server listening on http://{addr}");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received, exiting...");
}
