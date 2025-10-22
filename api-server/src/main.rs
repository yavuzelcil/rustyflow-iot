mod routes;

use axum::{Router, routing::get};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Basit config: PORT env var, yoksa 3000
    let port: u16 = std::env::var("APP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/ready",  get(routes::health::ready))
        .route("/",       get(routes::health::root));

    let addr = std::net::SocketAddr::from(([0,0,0,0], port));
    tracing::info!("api-server listening on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}