use crate::core::error::AppError;
use crate::core::providers::Provider;
use crate::core::state::SharedState;

pub struct NotificationService {
    state: SharedState,
}

impl NotificationService {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    #[allow(dead_code)]
    pub async fn send_fcm(&self, to: &str, title: &str, body: &str) -> Result<(), AppError> {
        self.state
            .fcm_provider
            .send_notification(to, title, body)
            .await
    }

    #[allow(dead_code)]
    pub async fn send_email(&self, to: &str, title: &str, body: &str) -> Result<(), AppError> {
        self.state
            .email_provider
            .send_notification(to, title, body)
            .await
    }

    pub async fn send_sms(&self, to: &str, otp: &str) -> Result<(), AppError> {
        self.state.sms_provider.send_otp(to, otp).await
    }

    pub async fn send_whatsapp(&self, to: &str, otp: &str) -> Result<(), AppError> {
        self.state.whatsapp_provider.send_otp(to, otp).await
    }
}
