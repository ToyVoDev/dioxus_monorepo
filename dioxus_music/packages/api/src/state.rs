use std::path::PathBuf;
use uuid::Uuid;
use crate::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub image_cache_dir: PathBuf,
    pub server_id: Uuid,
    pub music_dir: PathBuf,
}
