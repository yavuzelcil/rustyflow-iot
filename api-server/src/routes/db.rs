use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use crate::state::AppState;

#[derive(Serialize)]
struct DbHealth { db: &'static str }

pub async fn health(State(st): State<AppState>) -> (StatusCode, Json<DbHealth>) {
    if let Some(pool) = &st.db {
        // Basit check: SELECT 1
        let res = sqlx::query_scalar::<_, i64>("SELECT 1").fetch_one(pool).await;
        match res {
            Ok(_)  => (StatusCode::OK, Json(DbHealth { db: "up" })),
            Err(_) => (StatusCode::SERVICE_UNAVAILABLE, Json(DbHealth { db: "down" })),
        }
    } else {
        (StatusCode::NOT_IMPLEMENTED, Json(DbHealth { db: "disabled" }))
    }
}