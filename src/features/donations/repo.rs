use super::model::{CreateDonationRequest, Donation, DonationStatus};
use crate::core::error::AppError;
use crate::core::state::SharedState;
use chrono::Utc;

pub struct DonationRepository {
    state: SharedState,
}

impl DonationRepository {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    pub async fn create_donation(
        &self,
        payload: CreateDonationRequest,
    ) -> Result<Donation, AppError> {
        let record = Donation {
            id: None,
            project_id: payload.project_id,
            donor_name: payload.donor_name,
            donor_email: payload.donor_email,
            amount: payload.amount,
            currency: payload.currency,
            status: DonationStatus::Completed, // Simplification: assume successful for now
            message: payload.message,
            created_at: Utc::now().to_rfc3339(),
        };

        let created: Option<Donation> = self
            .state
            .db
            .create("donation")
            .content(record)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

        created.ok_or_else(|| {
            AppError::InternalServerError("Failed to return created donation".to_string())
        })
    }

    pub async fn get_all_donations(&self) -> Result<Vec<Donation>, AppError> {
        let donations: Vec<Donation> = self
            .state
            .db
            .select("donation")
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(donations)
    }
}
