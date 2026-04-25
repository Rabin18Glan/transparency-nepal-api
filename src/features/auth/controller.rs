use axum::{extract::State, Json};
use crate::state::SharedState;
use crate::error::AppError;
use super::model::{OtpRequest, OtpResponse, VerifyOtpRequest, VerifyOtpResponse};
use super::service::AuthService;
use validator::Validate;

pub async fn request_otp(
    State(state): State<SharedState>,
    Json(payload): Json<OtpRequest>,
) -> Result<Json<OtpResponse>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let service = AuthService::new(state);
    let result = service.request_otp(payload.phone_number, payload.channel).await?;

    Ok(Json(result))
}

pub async fn verify_otp(
    State(state): State<SharedState>,
    Json(payload): Json<VerifyOtpRequest>,
) -> Result<Json<VerifyOtpResponse>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let service = AuthService::new(state);
    let result = service.verify_otp(payload.phone_number, payload.otp).await?;

    Ok(Json(result))
}
