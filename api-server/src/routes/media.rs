use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

use crate::state::AppState;

// ---- Tipler ----
#[derive(Serialize, Clone, FromRow)]
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
    let id = Uuid::new_v4();
    
    if let Some(db) = &st.db {
        let item = sqlx::query_as::<_, Media>(
            "INSERT INTO media_datas (id, name, path) VALUES ($1, $2, $3) RETURNING id, name, path"
        )
        .bind(id)
        .bind(&body.name)
        .bind(&body.path)
        .fetch_one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok((StatusCode::CREATED, Json(item)))
    } else {
        // Fallback: in-memory store
        let item = Media {
            id,
            name: body.name,
            path: body.path,
        };
        let mut map = st.media_store.write().await;
        map.insert(item.id, item.clone());
        Ok((StatusCode::CREATED, Json(item)))
    }
}

// Read (list)
pub async fn list_media(State(st): State<AppState>) -> Result<Json<Vec<Media>>, StatusCode> {
    if let Some(db) = &st.db {
        let items = sqlx::query_as::<_, Media>("SELECT id, name, path FROM media_datas")
            .fetch_all(db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(items))
    } else {
        // Fallback: in-memory store
        let items = {
            let map = st.media_store.read().await;
            map.values().cloned().collect::<Vec<_>>()
        };
        Ok(Json(items))
    }
}

// Read (by id)
pub async fn get_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Media>, StatusCode> {
    if let Some(db) = &st.db {
        let item = sqlx::query_as::<_, Media>(
            "SELECT id, name, path FROM media_datas WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
        
        Ok(Json(item))
    } else {
        // Fallback: in-memory store
        let map = st.media_store.read().await;
        map.get(&id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
    }
}

// Update (partial)
pub async fn update_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(patch): Json<UpdateMedia>,
) -> Result<Json<Media>, StatusCode> {
    if let Some(db) = &st.db {
        // Mevcut kaydı al
        let current = sqlx::query_as::<_, Media>(
            "SELECT id, name, path FROM media_datas WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
        
        // Yeni değerleri hazırla (patch uygulanmış)
        let new_name = patch.name.unwrap_or(current.name);
        let new_path = patch.path.unwrap_or(current.path);
        
        // Güncelle
        let updated = sqlx::query_as::<_, Media>(
            "UPDATE media_datas SET name = $1, path = $2 WHERE id = $3 RETURNING id, name, path"
        )
        .bind(&new_name)
        .bind(&new_path)
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(updated))
    } else {
        // Fallback: in-memory store
        let mut map = st.media_store.write().await;
        let item = map.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
        if let Some(name) = patch.name { item.name = name; }
        if let Some(path) = patch.path { item.path = path; }
        Ok(Json(item.clone()))
    }
}   

// Delete
pub async fn delete_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    if let Some(db) = &st.db {
        let result = sqlx::query("DELETE FROM media_datas WHERE id = $1")
            .bind(id)
            .execute(db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        if result.rows_affected() > 0 {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        // Fallback: in-memory store
        let mut map = st.media_store.write().await;
        map.remove(&id).map(|_| StatusCode::NO_CONTENT).ok_or(StatusCode::NOT_FOUND)
    }
}