use crate::core::error::AppError;
use crate::core::providers::Provider;

pub struct FcmProvider {
    server_key: String,
}

impl FcmProvider {
    pub fn new(server_key: String) -> Self {
        Self { server_key }
    }
}

impl Provider for FcmProvider {
    async fn send_notification(
        &self,
        device_token: &str,
        title: &str,
        body: &str,
    ) -> Result<(), AppError> {
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "to": device_token,
            "notification": {
                "title": title,
                "body": body,
            }
        });

        let response = client
            .post("https://fcm.googleapis.com/fcm/send")
            .header("Authorization", format!("key={}", self.server_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("FCM request error: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".into());
            return Err(AppError::InternalServerError(format!(
                "FCM error: {}",
                error_text
            )));
        }

        Ok(())
    }
}
