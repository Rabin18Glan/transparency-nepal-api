use crate::core::error::AppError;
use crate::core::state::SharedState;
use bb8_redis::redis::AsyncCommands;

pub struct AuthRepository {
    state: SharedState,
}

impl AuthRepository {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    /// Stores an OTP in Redis with a 5-minute expiry.
    pub async fn store_otp(&self, phone_number: &str, otp: &str) -> Result<(), AppError> {
        let mut conn = self
            .state
            .cache
            .get()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Cache pool error: {}", e)))?;

        // Key format: otp:9841234567
        let key = format!("otp:{}", phone_number);

        let _: () = conn
            .set_ex(key, otp, 300) // 5 minutes expiry
            .await
            .map_err(|e| AppError::InternalServerError(format!("Cache error: {}", e)))?;

        Ok(())
    }

    /// Retrieves an OTP from Redis.
    pub async fn get_otp(&self, phone_number: &str) -> Result<Option<String>, AppError> {
        let mut conn = self
            .state
            .cache
            .get()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Cache pool error: {}", e)))?;

        let key = format!("otp:{}", phone_number);
        let otp: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Cache error: {}", e)))?;

        Ok(otp)
    }
}
