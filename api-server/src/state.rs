use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Config,
    // In-memory store: id -> Media
    pub media_store: Arc<RwLock<HashMap<Uuid, super::routes::media::Media>>>,
}