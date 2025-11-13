//! Yapılandırma Sistemi (Configuration)
//!
//! RustyFlow API sunucusu, ortam değişkenlerinden konfigürasyonu okur.
//! Desteklenen kaynaklar:
//! - .env dosyası (dotenvy ile)
//! - Sistem ortam değişkenleri
//! - Hardcoded fallback değerleri

use serde::Deserialize;

/// Sunucu yapılandırması
/// 
/// .env dosyasından veya ortam değişkenlerinden okunacak temel ayarlar.
/// 
/// Örnek .env dosyası:
/// ```ignore
/// APP_PORT=3000
/// DATABASE_URL=postgres://user:pass@localhost:5432/rustyflow
/// RUST_LOG=info
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// HTTP sunucusu dinlenecek port numarası
    /// 
    /// Varsayılan: 3000
    /// 
    /// Örnek: `APP_PORT=8080`
    #[serde(default = "default_port")]
    pub app_port: u16,

    /// PostgreSQL veritabanı bağlantı URL'i
    /// 
    /// Format: `postgres://[user[:password]@][host][:port][/dbname]`
    /// 
    /// Örnek: `DATABASE_URL=postgres://postgres:password@localhost:5432/rustyflow`
    /// 
    /// Eğer ayarlanmazsa, API in-memory fallback store kullanır.
    pub database_url: Option<String>,

    /// Logging seviyesi (tracing-subscriber için)
    /// 
    /// Geçerli değerler: error, warn, info, debug, trace
    /// 
    /// Varsayılan: "info"
    /// 
    /// Örnek: `RUST_LOG=debug`
    #[serde(default = "default_log")]
    pub log_level: String,
}

/// App port'un varsayılan değeri
fn default_port() -> u16 { 3000 }

/// Log seviyesinin varsayılan değeri
fn default_log() -> String { "info".into() }

impl Config {
    /// .env dosyasından ve ortam değişkenlerinden yapılandırmayı yükle
    /// 
    /// Yükleme sırası:
    /// 1. .env dosyasını oku (dotenvy ile)
    /// 2. Ortam değişkenlerini envy ile parse et
    /// 3. RUST_LOG özel işlemi (direkt ortam değişkenini kontrol et)
    /// 4. Fallback değerleri kullan
    /// 
    /// # Örnek
    /// ```ignore
    /// let config = Config::load();
    /// println!("Server port: {}", config.app_port);
    /// ```
    pub fn load() -> Self {
        // Step 1: .env dosyasını yükle (hata görmezden gel)
        let _ = dotenvy::dotenv();

        // Step 2: ENVY ile ortam değişkenlerini serde-compatible biçimde parse et
        let mut cfg: Config = envy::from_env().unwrap_or_else(|_| Config {
            app_port: default_port(),
            database_url: None,
            log_level: default_log(),
        });

        // Step 3: RUST_LOG ortam değişkenine özel davranış
        // (serde aracılığıyla yüklenmişse bile, direkt env var'ı tercih et)
        if let Ok(level) = std::env::var("RUST_LOG") {
            cfg.log_level = level;
        }

        cfg
    }

    /// Hassas bilgileri maskele ve herkese gösterebilecek hale getir
    /// 
    /// Veritabanı URL'sinin tam değerini herkese göstermek istemiyoruz
    /// (password gibi hassas bilgiler olabilir).
    /// 
    /// Bunun yerine "var mı yok mu" bilgisini döndür.
    /// 
    /// # Örnek
    /// ```ignore
    /// let config = Config::load();
    /// let safe = config.sanitized();
    /// // Response olarak güvenli hale getirilmiş config döndür
    /// ```
    pub fn sanitized(&self) -> SanitizedConfig {
        SanitizedConfig {
            app_port: self.app_port,
            has_database_url: self.database_url.is_some(),  // Sadece var/yok bilgisi
            log_level: self.log_level.clone(),
        }
    }
}

/// Güvenli yapılandırma (hassas bilgiler maskeli)
/// 
/// Bu struct, API client'larına döndürülebilir.
/// Veritabanı şifresi veya diğer hassas bilgiler içermez.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SanitizedConfig {
    /// HTTP sunucusu portu
    pub app_port: u16,
    /// Veritabanı bağlantısının konfigüre edilip edilmediği
    pub has_database_url: bool,
    /// Log seviyesi
    pub log_level: String,
}