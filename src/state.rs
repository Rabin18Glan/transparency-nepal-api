use std::sync::Arc;
use crate::config::database::Db;
use crate::config::cache::CachePool;
use crate::config::env::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub cache: CachePool,
    pub config: AppConfig,
    pub paseto: crate::common::auth::PasetoAuth,
}

pub type SharedState = Arc<AppState>;
