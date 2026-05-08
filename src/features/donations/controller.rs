use super::model::{CreateDonationRequest, Donation};
use super::service::DonationService;
use crate::core::error::AppError;
use crate::core::state::SharedState;
use crate::shared::auth::AuthUser;
use axum::{extract::State, Json};
use validator::Validate;

pub async fn create_donation(
    State(state): State<SharedState>,
    _user: AuthUser,
    Json(payload): Json<CreateDonationRequest>,
) -> Result<Json<Donation>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let service = DonationService::new(state);
    let result = service.create_donation(payload).await?;

    Ok(Json(result))
}

pub async fn list_donations(
    State(state): State<SharedState>,
) -> Result<Json<Vec<Donation>>, AppError> {
    let service = DonationService::new(state);
    let donations = service.list_donations().await?;

    Ok(Json(donations))
}
