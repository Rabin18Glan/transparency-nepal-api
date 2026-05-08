use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Donation {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::features::projects::model::serialize_id"
    )]
    pub id: Option<Thing>,
    pub project_id: String,
    pub donor_name: String,
    pub donor_email: String,
    pub amount: f64,
    pub currency: String,
    pub status: DonationStatus,
    pub message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DonationStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDonationRequest {
    pub project_id: String,
    pub donor_name: String,
    pub donor_email: String,
    pub amount: f64,
    pub currency: String,
    pub message: Option<String>,
}
