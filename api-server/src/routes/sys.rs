use axum::{extract::State, response::IntoResponse, Json};
//use crate::config::SanitizedConfig;
use crate::state::AppState;

pub async fn config(State(st): State<AppState>) -> impl IntoResponse {
    Json(st.cfg.sanitized())
}