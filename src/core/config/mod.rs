pub mod cache;
pub mod database;
pub mod env;

use crate::core::state::AppState;
use std::sync::Arc;

pub async fn initialize_app_state() -> Arc<AppState> {
    // 1. Load Environment
    let config = env::AppConfig::from_env();

    // 2. Initialize Database
    let db = database::init_surreal(&config.surreal_url).await;

    // 3. Initialize Cache
    let cache = cache::init_cache(&config.cache_url).await;

    // 4. Initialize PasetoAuth
    let paseto = crate::shared::auth::PasetoAuth::new(&config.paseto_secret)
        .expect("Failed to initialize PASETO Auth");

    // 5. Initialize Providers
    let sms_provider = Arc::new(crate::shared::providers::sms::SmsProvider::new(
        config.sparrow_sms_token.clone(),
        config.sparrow_sms_sender.clone(),
    ));
    let whatsapp_provider = Arc::new(crate::shared::providers::whatsapp::WhatsAppProvider::new(
        config.whatsapp_access_token.clone(),
        config.whatsapp_phone_number_id.clone(),
    ));
    let fcm_provider = Arc::new(crate::shared::providers::fcm::FcmProvider::new(
        config.fcm_server_key.clone(),
    ));
    let email_provider = Arc::new(crate::shared::providers::email::EmailProvider::new(
        config.smtp_host.clone(),
        config.smtp_port,
        config.smtp_user.clone(),
        config.smtp_pass.clone(),
    ));

    // 6. Initialize Broadcast Channel for real-time updates
    let (tx, _) = tokio::sync::broadcast::channel(100);

    // 7. Wrap in AppState
    Arc::new(AppState {
        db,
        cache,
        config: config.clone(),
        paseto,
        tx,
        sms_provider,
        whatsapp_provider,
        fcm_provider,
        email_provider,
    })
}
