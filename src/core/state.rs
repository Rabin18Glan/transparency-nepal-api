use crate::core::config::cache::CachePool;
use crate::core::config::database::Db;
use crate::core::config::env::AppConfig;
use crate::shared::auth::PasetoAuth;
use crate::shared::providers::{
    email::EmailProvider, fcm::FcmProvider, sms::SmsProvider, whatsapp::WhatsAppProvider,
};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub cache: CachePool,
    pub config: AppConfig,
    pub paseto: PasetoAuth,
    pub tx: broadcast::Sender<String>,

    // Providers
    pub sms_provider: Arc<SmsProvider>,
    pub whatsapp_provider: Arc<WhatsAppProvider>,
    pub fcm_provider: Arc<FcmProvider>,
    pub email_provider: Arc<EmailProvider>,
}

pub type SharedState = Arc<AppState>;
