//! Media CRUD Endpoint'leri
//!
//! Fotoğraf, video ve diğer media dosyalarının yönetimi için REST endpoint'leri.
//! PostgreSQL'e veya in-memory store'a kayıt yapılır (fallback).
//!
//! # Yapı
//! - Media: shared-types'tan import edilen media nesnesi
//! - NewMedia: shared-types'tan import edilen yeni media request
//! - UpdateMedia: shared-types'tan import edilen güncelleme request
//!
//! # Endpoint'ler
//! - POST /v1/media - Yeni media oluştur
//! - GET /v1/media - Tüm medya listele
//! - GET /v1/media/{id} - Belirli bir medyayı al
//! - PUT /v1/media/{id} - Medyayı güncelle (partial)
//! - DELETE /v1/media/{id} - Medyayı sil

use axum::{extract::{Path, State}, http::StatusCode, Json};
use uuid::Uuid;

use crate::state::AppState;

// shared-types'tan Media tiplerini import et
// Artık kendi Media struct'ımız yok, merkezi shared-types'ı kullanıyoruz
use shared_types::{Media, NewMedia, UpdateMedia};

// ============================================================================
// HTTP HANDLER FONKSİYONLARI (HTTP HANDLERS)
// ============================================================================

/// Yeni bir medya nesnesi oluştur
/// 
/// # HTTP
/// `POST /v1/media`
/// 
/// # Request Body
/// ```json
/// {
///   "name": "photo.jpg",
///   "path": "/uploads/2024/photo.jpg"
/// }
/// ```
/// 
/// # Response (201 Created)
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "name": "photo.jpg",
///   "path": "/uploads/2024/photo.jpg"
/// }
/// ```
/// 
/// # Detay
/// 1. Yeni bir UUID v4 oluştur
/// 2. Eğer PostgreSQL bağlıysa: INSERT query'si çalıştır
/// 3. Yoksa: In-memory HashMap'e ekle
/// 4. 201 (CREATED) status ile response dön
pub async fn create_media(
    State(st): State<AppState>,
    Json(body): Json<NewMedia>,
) -> Result<(StatusCode, Json<Media>), StatusCode> {
    if let Some(db) = &st.db {
        // ===== PostgreSQL Yolu =====
        let item = sqlx::query_as::<_, Media>(
            "INSERT INTO media_datas (id, name, path, mime_type, size_bytes, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) 
             RETURNING id, name, path, mime_type, size_bytes, created_at, updated_at"
        )
        .bind(Uuid::new_v4())
        .bind(&body.name)
        .bind(&body.path)
        .bind(&body.mime_type)
        .bind(body.size_bytes)
        .fetch_one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok((StatusCode::CREATED, Json(item)))
    } else {
        // ===== In-Memory Fallback =====
        let item = Media::new(body.name, body.path, body.mime_type, body.size_bytes);
        let mut map = st.media_store.write().await;
        map.insert(item.id, item.clone());
        Ok((StatusCode::CREATED, Json(item)))
    }
}

/// Tüm media nesnelerini listele
/// 
/// # HTTP
/// `GET /v1/media`
/// 
/// # Response (200 OK)
/// ```json
/// [
///   {
///     "id": "550e8400-e29b-41d4-a716-446655440000",
///     "name": "photo1.jpg",
///     "path": "/uploads/photo1.jpg"
///   },
///   {
///     "id": "550e8400-e29b-41d4-a716-446655440001",
///     "name": "photo2.jpg",
///     "path": "/uploads/photo2.jpg"
///   }
/// ]
/// ```
/// 
/// # Detay
/// 1. Eğer PostgreSQL bağlıysa: SELECT * FROM media_datas
/// 2. Yoksa: In-memory HashMap'teki tüm değerleri dön
pub async fn list_media(State(st): State<AppState>) -> Result<Json<Vec<Media>>, StatusCode> {
    if let Some(db) = &st.db {
        // ===== PostgreSQL Yolu =====
        let items = sqlx::query_as::<_, Media>(
            "SELECT id, name, path, mime_type, size_bytes, created_at, updated_at FROM media_datas"
        )
            .fetch_all(db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(items))
    } else {
        // ===== In-Memory Fallback =====
        let items = {
            let map = st.media_store.read().await;
            map.values().cloned().collect::<Vec<_>>()
        };
        Ok(Json(items))
    }
}

/// Belirli bir media nesnesini ID'si ile al
/// 
/// # HTTP
/// `GET /v1/media/{id}`
/// 
/// # Path Parameter
/// - `id` (UUID): Media nesnesinin benzersiz tanımlayıcısı
/// 
/// # Response (200 OK)
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "name": "photo.jpg",
///   "path": "/uploads/2024/photo.jpg"
/// }
/// ```
/// 
/// # Error Responses
/// - 404 Not Found: ID bulunamadı
/// - 500 Internal Server Error: Database hatası
pub async fn get_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Media>, StatusCode> {
    if let Some(db) = &st.db {
        // ===== PostgreSQL Yolu =====
        let item = sqlx::query_as::<_, Media>(
            "SELECT id, name, path, mime_type, size_bytes, created_at, updated_at FROM media_datas WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)  // Sonuç: Option<Media>
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;  // Eğer None ise 404 dön
        
        Ok(Json(item))
    } else {
        // ===== In-Memory Fallback =====
        let map = st.media_store.read().await;
        map.get(&id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
    }
}

/// Media nesnesini güncelle (partial update)
/// 
/// # HTTP
/// `PUT /v1/media/{id}`
/// 
/// # Path Parameter
/// - `id` (UUID): Güncellenecek media nesnesinin ID'si
/// 
/// # Request Body (Partial)
/// ```json
/// {
///   "name": "new-name.jpg",
///   "path": null
/// }
/// ```
/// 
/// # Response (200 OK)
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "name": "new-name.jpg",
///   "path": "/uploads/2024/photo.jpg"
/// }
/// ```
/// 
/// # Update Mantığı
/// 1. Mevcut kaydı al
/// 2. Gönderilen alanları (null olmayan) yeni değerlerle değiştir
/// 3. Eski değerleri koru (null ise)
/// 4. Güncellenmiş kaydı döndür
pub async fn update_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(patch): Json<UpdateMedia>,
) -> Result<Json<Media>, StatusCode> {
    if let Some(db) = &st.db {
        // ===== PostgreSQL Yolu =====
        
        // Step 1: Mevcut kaydı al
        let mut current = sqlx::query_as::<_, Media>(
            "SELECT id, name, path, mime_type, size_bytes, created_at, updated_at FROM media_datas WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
        
        // Step 2: Patch'i uygula
        current.apply_update(&patch);
        
        // Step 3: Database'i güncelle
        let updated = sqlx::query_as::<_, Media>(
            "UPDATE media_datas SET name = $1, path = $2, mime_type = $3, size_bytes = $4, updated_at = NOW() 
             WHERE id = $5 
             RETURNING id, name, path, mime_type, size_bytes, created_at, updated_at"
        )
        .bind(&current.name)
        .bind(&current.path)
        .bind(&current.mime_type)
        .bind(current.size_bytes)
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        Ok(Json(updated))
    } else {
        // ===== In-Memory Fallback =====
        let mut map = st.media_store.write().await;
        let item = map.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
        item.apply_update(&patch);
        Ok(Json(item.clone()))
    }
}   

/// Bir media nesnesini sil
/// 
/// # HTTP
/// `DELETE /v1/media/{id}`
/// 
/// # Path Parameter
/// - `id` (UUID): Silinecek media nesnesinin ID'si
/// 
/// # Response (204 No Content)
/// Başarılı silme için boş response dön (HTTP 204)
/// 
/// # Error Responses
/// - 404 Not Found: ID bulunamadı
/// - 500 Internal Server Error: Database hatası
pub async fn delete_media(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    if let Some(db) = &st.db {
        // ===== PostgreSQL Yolu =====
        let result = sqlx::query("DELETE FROM media_datas WHERE id = $1")
            .bind(id)
            .execute(db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // rows_affected() = 0 ise, kayıt yoktu
        if result.rows_affected() > 0 {
            Ok(StatusCode::NO_CONTENT)  // 204
        } else {
            Err(StatusCode::NOT_FOUND)   // 404
        }
    } else {
        // ===== In-Memory Fallback =====
        let mut map = st.media_store.write().await;
        map.remove(&id).map(|_| StatusCode::NO_CONTENT).ok_or(StatusCode::NOT_FOUND)
    }
}