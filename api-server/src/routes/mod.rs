//! HTTP Route Handler'ları
//!
//! Tüm API endpoint'lerinin handler'larını içerir.
//! Modüler yapı sayesinde her endpoint grubu ayrı dosyada tutulur.

pub mod health;   // Sağlık kontrol endpoint'leri (/, /health, /ready)
pub mod sys;      // Sistem endpoint'leri (/v1/config)
pub mod media;    // Media CRUD endpoint'leri (/v1/media/*)
pub mod db;       // Database endpoint'leri (/db/*) 