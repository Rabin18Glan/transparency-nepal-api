pub mod env;
pub mod database;
pub mod cache;
pub mod providers;

use std::sync::Arc;
use crate::state::AppState;

pub async fn initialize_app_state() -> Arc<AppState> {
    // 1. Load Environment
    let config = env::AppConfig::from_env();

    // 2. Initialize Database
    let db = database::init_surreal(&config.surreal_url).await;

    // 3. Initialize Cache
    let cache = cache::init_cache(&config.cache_url).await;

    // 4. Initialize PasetoAuth
    let paseto = crate::common::auth::PasetoAuth::new(&config.paseto_secret)
        .expect("Failed to initialize PASETO Auth");

    // 5. Wrap in AppState
    Arc::new(AppState {
        db,
        cache,
        config: config.clone(),
        paseto,
    })
}
