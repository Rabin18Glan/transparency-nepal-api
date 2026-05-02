use crate::state::SharedState;

use super::model::Contribution;

#[derive(Clone)]
pub struct ContributionRepository {
    state: SharedState,
}

impl ContributionRepository {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    pub async fn create(&self, contribution: Contribution) -> Result<Contribution, surrealdb::Error> {
        let created: Option<Contribution> = self.state.db
            .create("contribution")
            .content(contribution)
            .await?;
        
        Ok(created.unwrap())
    }

    pub async fn list(&self) -> Result<Vec<Contribution>, surrealdb::Error> {
        let contributions: Vec<Contribution> = self.state.db.select("contribution").await?;
        Ok(contributions)
    }
}
