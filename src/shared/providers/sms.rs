use crate::core::error::AppError;
use crate::core::providers::Provider;

pub struct SmsProvider {
    token: String,
    sender_id: String,
}

impl SmsProvider {
    pub fn new(token: String, sender_id: String) -> Self {
        Self { token, sender_id }
    }
}

impl Provider for SmsProvider {
    async fn send_otp(&self, phone_number: &str, otp: &str) -> Result<(), AppError> {
        if cfg!(debug_assertions) {
            tracing::info!("[DEBUG] SMS skipped for {} — OTP: {}", phone_number, otp);
            return Ok(());
        }
        let payload = serde_json::json!({
            "token": self.token,
            "from": self.sender_id,
            "to": phone_number,
            "text": format!("Your Gorkhas OTP is: {}. Valid for 5 minutes.", otp),
        });

        reqwest::Client::new()
            .post("https://api.sparrowsms.com/v2/sms/")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("SMS dispatch failed: {}", e)))?;

        tracing::info!("SMS dispatched to {}", phone_number);
        Ok(())
    }
}
