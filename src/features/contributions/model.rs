use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contribution {
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "crate::features::projects::model::serialize_id")]
    pub id: Option<Thing>,
    pub project_id: String,
    pub user_id: String,
    pub contribution_type: ContributionType,
    pub description: String,
    pub media_urls: Vec<String>,
    pub status: ContributionStatus,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ContributionType {
    Proof,
    IrregularityReport,
    Volunteer,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ContributionStatus {
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateContributionRequest {
    pub project_id: String,
    pub contribution_type: ContributionType,
    pub description: String,
    #[serde(default)]
    pub media_urls: Vec<String>,
}
