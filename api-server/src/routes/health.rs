use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
struct Health { status: &'static str }

pub async fn root() -> &'static str {
    "RustyFlow API — OK"
}

pub async fn health() -> impl IntoResponse {
    Json(Health { status: "ok" })
}

pub async fn ready() -> impl IntoResponse {
    // ileride: DB bağlantısı, mq bağlı mı gibi kontroller.
    Json(Health { status: "ready" })
}