use serde::Deserialize;

/// Ortam değişkenlerinden okunacak temel yapılandırma.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Sunucu portu (ör: 3000)
    #[serde(default = "default_port")]
    pub app_port: u16,

    /// Opsiyonel: İleride SQLx için kullanılacak
    pub database_url: Option<String>,

    /// Log seviyesi (örn. info, debug)
    #[serde(default = "default_log")]
    pub log_level: String,
}

fn default_port() -> u16 { 3000 }
fn default_log() -> String { "info".into() }

impl Config {
    /// .env + ortam değişkenlerinden konfigürasyonu yükle.
    pub fn load() -> Self {
        let _ = dotenvy::dotenv();

        // ENVY = serde destekli env parser
        let mut cfg: Config = envy::from_env().unwrap_or_else(|_| Config {
            app_port: default_port(),
            database_url: None,
            log_level: default_log(),
        });

        // RUST_LOG ortam değişkeni varsa onu tercih et
        if let Ok(level) = std::env::var("RUST_LOG") {
            cfg.log_level = level;
        }

        cfg
    }

    /// Dışarıya gösterirken hassas alanları maskeler
    pub fn sanitized(&self) -> SanitizedConfig {
        SanitizedConfig {
            app_port: self.app_port,
            has_database_url: self.database_url.is_some(),
            log_level: self.log_level.clone(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SanitizedConfig {
    pub app_port: u16,
    pub has_database_url: bool,
    pub log_level: String,
}