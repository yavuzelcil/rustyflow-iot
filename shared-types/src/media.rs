//! Media Types
//!
//! Fotoğraf, video ve diğer medya dosyalarını temsil eden veri yapıları.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(feature = "sqlx-support")]
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// Medya nesnesi - Veritabanında depolanan gösterim
/// 
/// Bir medya dosyasının (fotoğraf, video vb.) metadata'sını tutar.
/// UUID tabanlı benzersiz tanımlayıcıya sahiptir.
/// 
/// # Alanlar
/// 
/// - `id`: UUID v4 benzersiz tanımlayıcı
/// - `name`: Medya dosyasının adı (örn: "vacation.jpg")
/// - `path`: Depolama konumu (örn: "/uploads/2024/vacation.jpg")
/// - `mime_type`: MIME type (örn: "image/jpeg")
/// - `size_bytes`: Dosya boyutu (bytes cinsinden)
/// - `created_at`: Oluşturulma tarihi (ISO 8601)
/// - `updated_at`: Son güncellenme tarihi (ISO 8601)
/// 
/// # Serializasyon
/// 
/// - `Serialize`: JSON response'ları için
/// - `Deserialize`: JSON request'ler için
/// - `FromRow`: SQLx ile PostgreSQL satırlarından otomatik dönüştürme
/// - `Clone`: Request arasında verileri kopyalamak için
/// 
/// # Örnek JSON
/// ```json
/// {
///   "id": "550e8400-e29b-41d4-a716-446655440000",
///   "name": "avatar.png",
///   "path": "/uploads/users/123/avatar.png",
///   "mime_type": "image/png",
///   "size_bytes": 24576,
///   "created_at": "2024-11-13T21:30:00Z",
///   "updated_at": "2024-11-13T21:30:00Z"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx-support", derive(FromRow))]
pub struct Media {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Yeni medya oluştururken gönderilen request body
/// 
/// Client tarafından POST /v1/media'ya JSON olarak gönderilir.
/// 
/// # Örnek Request
/// ```json
/// {
///   "name": "vacation.jpg",
///   "path": "/uploads/photos/2024/vacation.jpg",
///   "mime_type": "image/jpeg",
///   "size_bytes": 2048576
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct NewMedia {
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size_bytes: i64,
}

/// Medya güncellerken gönderilen request body (partial update)
/// 
/// Sadece değiştirmek istenen alanları göndermek yeterli.
/// null değerler mevcut değeri korur.
/// 
/// # Örnek 1: Sadece adı güncelle
/// ```json
/// {
///   "name": "new-name.jpg"
/// }
/// ```
/// 
/// # Örnek 2: Çoklu alanları güncelle
/// ```json
/// {
///   "name": "new-name.jpg",
///   "path": "/uploads/photos/archive/new-location.jpg"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct UpdateMedia {
    pub name: Option<String>,
    pub path: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
}

impl Media {
    /// Yeni bir Media nesnesi oluştur
    /// 
    /// # Parametreler
    /// 
    /// - `name`: Dosya adı
    /// - `path`: Depolama yolu
    /// - `mime_type`: MIME type
    /// - `size_bytes`: Dosya boyutu
    /// 
    /// # Detay
    /// 
    /// - ID: Yeni UUID v4 oluşturulur
    /// - Timestamps: Şu anki UTC zamanı kullanılır
    pub fn new(name: String, path: String, mime_type: String, size_bytes: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            path,
            mime_type,
            size_bytes,
            created_at: now,
            updated_at: now,
        }
    }

    /// Media nesnesini UpdateMedia ile güncelle (partial)
    /// 
    /// null olmayan değerleri günceller, null olanları korur.
    pub fn apply_update(&mut self, update: &UpdateMedia) {
        if let Some(name) = &update.name {
            self.name = name.clone();
        }
        if let Some(path) = &update.path {
            self.path = path.clone();
        }
        if let Some(mime_type) = &update.mime_type {
            self.mime_type = mime_type.clone();
        }
        if let Some(size_bytes) = update.size_bytes {
            self.size_bytes = size_bytes;
        }
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_media() {
        let media = Media::new(
            "test.jpg".to_string(),
            "/uploads/test.jpg".to_string(),
            "image/jpeg".to_string(),
            1024,
        );
        
        assert_eq!(media.name, "test.jpg");
        assert_eq!(media.size_bytes, 1024);
        assert!(!media.id.to_string().is_empty());
    }

    #[test]
    fn test_apply_update() {
        let mut media = Media::new(
            "test.jpg".to_string(),
            "/uploads/test.jpg".to_string(),
            "image/jpeg".to_string(),
            1024,
        );
        
        let update = UpdateMedia {
            name: Some("new-name.jpg".to_string()),
            path: None,
            mime_type: None,
            size_bytes: None,
        };
        
        media.apply_update(&update);
        assert_eq!(media.name, "new-name.jpg");
        assert_eq!(media.path, "/uploads/test.jpg"); // Değiştirilmedi
    }
}
