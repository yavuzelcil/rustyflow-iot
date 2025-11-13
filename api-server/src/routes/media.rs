//! Media CRUD Endpoint'leri
//!
//! Fotoğraf, video ve diğer media dosyalarının yönetimi için REST endpoint'leri.
//! PostgreSQL'e veya in-memory store'a kayıt yapılır (fallback).
//!
//! # Yapı
//! - Media: Yayınlanan media nesnesinin yapısı
//! - NewMedia: Yeni media oluştururken gönderilen veri
//! - UpdateMedia: Media güncellerken gönderilen veri (partial update)
//!
//! # Endpoint'ler
//! - POST /v1/media - Yeni media oluştur
//! - GET /v1/media - Tüm medya listele
//! - GET /v1/media/{id} - Belirli bir medyayı al
//! - PUT /v1/media/{id} - Medyayı güncelle (partial)
//! - DELETE /v1/media/{id} - Medyayı sil

use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

use crate::state::AppState;

// ============================================================================
// VERİ TİPLERİ (DATA STRUCTURES)
// ============================================================================

/// Medya nesnesi - Veritabanında depolanan gösterim
/// 
/// # Alanlar
/// - `id`: Benzersiz UUID tanımlayıcı (UUID v4)
/// - `name`: Medya dosyasının adı (örn: "profile.jpg")
/// - `path`: Dosya sistemi veya S3 yolu (örn: "/uploads/2024/profile.jpg")
/// 
/// # Serializasyon
/// - `Serialize`: JSON response'ları için
/// - `FromRow`: SQLx ile database satırlarından otomatik dönüştürme
/// - `Clone`: Request arasında verileri kopyalamak için
/// 
/// # Örnek
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "name": "avatar.png",
///   "path": "/uploads/users/123/avatar.png"
/// }
/// ```
#[derive(Serialize, Clone, FromRow)]
pub struct Media {
    pub id: Uuid,
    pub name: String,
    pub path: String,
}

/// Yeni medya oluştururken gönderilen request body
/// 
/// Client POST /v1/media'ya bu yapıyı JSON olarak gönderir.
/// 
/// # Örnek
/// ```json
/// {
///   "name": "vacation.jpg",
///   "path": "/uploads/photos/2024/vacation.jpg"
/// }
/// ```
#[derive(Deserialize)]
pub struct NewMedia {
    pub name: String,
    pub path: String,
}

/// Medya güncellerken gönderilen request body (partial update)
/// 
/// Client PUT /v1/media/{id}'ye bu yapıyı JSON olarak gönderir.
/// Sadece değiştirmek istenen alanları göndermek yeterli.
/// 
/// # Örnek 1: Sadece adı güncelle
/// ```json
/// {
///   "name": "new-name.jpg",
///   "path": null
/// }
/// ```
/// 
/// # Örnek 2: Sadece yolu güncelle
/// ```json
/// {
///   "name": null,
///   "path": "/uploads/photos/new/location.jpg"
/// }
/// ```
#[derive(Deserialize)]
pub struct UpdateMedia {
    pub name: Option<String>,
    pub path: Option<String>,
}

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
    let id = Uuid::new_v4();
    
    if let Some(db) = &st.db {
        // ===== PostgreSQL Yolu =====
        // SQL: INSERT INTO media_datas (id, name, path) VALUES (...) RETURNING *
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
        // ===== In-Memory Fallback =====
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
        let items = sqlx::query_as::<_, Media>("SELECT id, name, path FROM media_datas")
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
            "SELECT id, name, path FROM media_datas WHERE id = $1"
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
        let current = sqlx::query_as::<_, Media>(
            "SELECT id, name, path FROM media_datas WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
        
        // Step 2: Patch değerlerini veya eski değerleri kullan
        let new_name = patch.name.unwrap_or(current.name);
        let new_path = patch.path.unwrap_or(current.path);
        
        // Step 3: Database'i güncelle
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
        // ===== In-Memory Fallback =====
        let mut map = st.media_store.write().await;
        let item = map.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
        
        // Patch uygula (Option::unwrap_or mantığı)
        if let Some(name) = patch.name { item.name = name; }
        if let Some(path) = patch.path { item.path = path; }
        
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