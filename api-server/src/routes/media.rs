use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

// ---- Tipler ----
#[derive(Serialize, Clone)]
pub struct Media {
    pub id: Uuid,
    pub name: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct NewMedia {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct UpdateMedia {
    pub name: Option<String>,
    pub path: Option<String>,
}

// ---- Handlers ----

// Create
pub async fn create_media(
    State(st): State<AppState>,
    Json(body): Json<NewMedia>,
) -> Result<(StatusCode, Json<Media>), StatusCode> {
    let item = Media {
        id: Uuid::new_v4(),
        name: body.name,
        path: body.path,
    };
    {
        // Kilidi kısa tut: sadece ekleme anında
        let mut map = st.media_store.write().await;
        map.insert(item.id, item.clone());
    }
    Ok((StatusCode::CREATED, Json(item)))
}

// Read (list)
pub async fn list_media(State(st): State<AppState>) -> Result<Json<Vec<Media>>, StatusCode> {
    let items = {
        let map = st.media_store.read().await;
        map.values().cloned().collect::<Vec<_>>()
    };
    Ok(Json(items))
}

// Read (by id)
pub async fn get_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Media>, StatusCode> {
    let maybe = {
        let map = st.media_store.read().await;
        map.get(&id).cloned()
    };
    maybe.map(Json).ok_or(StatusCode::NOT_FOUND)
}

// Update (partial)
pub async fn update_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(patch): Json<UpdateMedia>,
) -> Result<Json<Media>, StatusCode> {
    let mut map = st.media_store.write().await;
    let item = map.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(name) = patch.name { item.name = name; }
    if let Some(path) = patch.path { item.path = path; }
    Ok(Json(item.clone()))
}   

// Delete
pub async fn delete_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut map = st.media_store.write().await;
    map.remove(&id).map(|_| StatusCode::NO_CONTENT).ok_or(StatusCode::NOT_FOUND)
}