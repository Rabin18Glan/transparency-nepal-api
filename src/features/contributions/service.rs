use std::sync::Arc;
use chrono::Utc;

use super::model::{Contribution, ContributionStatus, CreateContributionRequest};
use super::repo::ContributionRepository;

#[derive(Clone)]
pub struct ContributionService {
    repo: Arc<ContributionRepository>,
}

impl ContributionService {
    pub fn new(repo: Arc<ContributionRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_contribution(&self, user_id: String, req: CreateContributionRequest) -> Result<Contribution, String> {
        let new_contribution = Contribution {
            id: None,
            project_id: req.project_id,
            user_id,
            contribution_type: req.contribution_type,
            description: req.description,
            media_urls: req.media_urls,
            status: ContributionStatus::Pending,
            created_at: Utc::now().to_rfc3339(),
        };

        self.repo.create(new_contribution).await.map_err(|e| e.to_string())
    }

    pub async fn list_contributions(&self) -> Result<Vec<Contribution>, String> {
        self.repo.list().await.map_err(|e| e.to_string())
    }
}
