//! Error Types and Handling
//!
//! RustyFlow projesi boyunca kullanılan hata tipleri.
//! `thiserror` crate'i ile structured error handling.

use thiserror::Error;

/// RustyFlow hata tipi
/// 
/// Tüm servislerde kullanılan merkezi error tipi.
/// Detaylı error mesajları ve kategorilendirme sağlar.
#[derive(Debug, Error)]
pub enum Error {
    /// Veritabanı işleminde hata
    #[error("Database error: {0}")]
    Database(String),

    /// Medya dosyası bulunamadı
    #[error("Media not found: {0}")]
    MediaNotFound(String),

    /// Sensor verisi bulunamadı
    #[error("Sensor not found: {0}")]
    SensorNotFound(String),

    /// Geçersiz UUID formatı
    #[error("Invalid UUID format: {0}")]
    InvalidUuid(String),

    /// MQTT bağlantı hatası
    #[error("MQTT connection error: {0}")]
    MqttError(String),

    /// Serializasyon hatası
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Geçersiz parametre
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// İç sunucu hatası
    #[error("Internal server error: {0}")]
    InternalError(String),

    /// Yetkilendirme hatası
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Erişim reddedildi
    #[error("Forbidden: {0}")]
    Forbidden(String),
}

/// RustyFlow Result tipi
/// 
/// Tüm servislerde kullanılan Result alias'ı.
/// Success = T, Failure = Error
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// HTTP status code'u döndür
    /// 
    /// Error'ü HTTP response'unda kullanırken uygun status code döndür.
    pub fn status_code(&self) -> u16 {
        match self {
            Error::MediaNotFound(_) | Error::SensorNotFound(_) => 404,
            Error::InvalidUuid(_) | Error::InvalidParameter(_) => 400,
            Error::Unauthorized(_) => 401,
            Error::Forbidden(_) => 403,
            Error::Database(_) | Error::MqttError(_) => 503,
            Error::SerializationError(_) | Error::InternalError(_) => 500,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(Error::MediaNotFound("1".to_string()).status_code(), 404);
        assert_eq!(Error::InvalidParameter("x".to_string()).status_code(), 400);
        assert_eq!(Error::Unauthorized("test".to_string()).status_code(), 401);
        assert_eq!(Error::Database("test".to_string()).status_code(), 503);
    }
}
