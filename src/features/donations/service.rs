use super::model::{CreateDonationRequest, Donation};
use super::repo::DonationRepository;
use crate::core::error::AppError;
use crate::core::state::SharedState;

pub struct DonationService {
    repo: DonationRepository,
}

impl DonationService {
    pub fn new(state: SharedState) -> Self {
        Self {
            repo: DonationRepository::new(state),
        }
    }

    pub async fn create_donation(
        &self,
        payload: CreateDonationRequest,
    ) -> Result<Donation, AppError> {
        tracing::info!("Processing donation from: {}", payload.donor_name);
        self.repo.create_donation(payload).await
    }

    pub async fn list_donations(&self) -> Result<Vec<Donation>, AppError> {
        tracing::info!("Retrieving all donations");
        self.repo.get_all_donations().await
    }
}
