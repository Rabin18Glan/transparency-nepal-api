use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub expected_budget: f64,
    pub status: ProjectStatus,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    Halted,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 5, message = "Title must be at least 5 characters"))]
    pub title: String,
    #[validate(length(min = 10, message = "Description must provide detail"))]
    pub description: String,
    pub expected_budget: f64,
    pub latitude: f64,
    pub longitude: f64,
}

