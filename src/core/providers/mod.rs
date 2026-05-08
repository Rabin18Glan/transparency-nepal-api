use crate::core::error::AppError;

pub trait Provider: Send + Sync {
    async fn send_otp(&self, _to: &str, _otp: &str) -> Result<(), AppError> {
        Err(AppError::InternalServerError(
            "OTP not supported for this provider".to_string(),
        ))
    }

    async fn send_notification(
        &self,
        _to: &str,
        _title: &str,
        _body: &str,
    ) -> Result<(), AppError> {
        Err(AppError::InternalServerError(
            "Notification not supported for this provider".to_string(),
        ))
    }
}
