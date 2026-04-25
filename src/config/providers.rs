use crate::error::AppError;

pub struct SmsProvider {
    token: String,
    sender_id: String,
}

impl SmsProvider {
    pub fn new(token: String, sender_id: String) -> Self {
        Self { token, sender_id }
    }

    pub async fn send_otp(&self, phone_number: &str, otp: &str) -> Result<(), AppError> {
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

pub struct WhatsAppProvider {
    access_token: String,
    phone_number_id: String,
}

impl WhatsAppProvider {
    pub fn new(access_token: String, phone_number_id: String) -> Self {
        Self { access_token, phone_number_id }
    }

    pub async fn send_otp(&self, phone_number: &str, otp: &str) -> Result<(), AppError> {
        if cfg!(debug_assertions) {
            tracing::info!("[DEBUG] WhatsApp skipped for {} — OTP: {}", phone_number, otp);
            return Ok(());
        }
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "to": phone_number,
            "type": "text",
            "text": {
                "body": format!("Your Gorkhas OTP is: {}. Valid for 5 minutes.", otp)
            }
        });

        reqwest::Client::new()
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("WhatsApp dispatch failed: {}", e)))?;

        tracing::info!("WhatsApp dispatched to {}", phone_number);
        Ok(())
    }
}
