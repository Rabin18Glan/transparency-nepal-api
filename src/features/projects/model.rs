use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "crate::features::projects::model::serialize_id")]
    pub id: Option<Thing>,
    pub project_code: String,
    pub title: String,
    pub description: String,
    pub tier_level: ProjectTier,
    pub sector: String,
    pub implementation_modality: String,
    pub status: ProjectStatus,
    pub planned_start_date: Option<String>,
    pub estimated_completion_date: Option<String>,
    pub estimated_total_cost: f64,
    
    // Geographic data
    pub location: ProjectLocation,
    
    // Detailed sections (Nested for simplicity in this vertical slice)
    #[serde(default)]
    pub personnel: Vec<ProjectPersonnel>,
    #[serde(default)]
    pub financial_flows: Vec<FinancialFlow>,
    #[serde(default)]
    pub results_framework: Vec<Metric>,
    #[serde(default)]
    pub communications_log: Vec<CommunicationLog>,
    #[serde(default)]
    pub opinions: Vec<ProjectOpinion>,

    #[serde(default)]
    pub reactions: Vec<ProjectReaction>,

    pub created_at: String,
}

pub fn serialize_id<S>(id: &Option<Thing>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match id {
        Some(t) => s.serialize_str(&t.to_string()),
        None => s.serialize_none(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectOpinion {
    pub user_id: String,
    pub user_name: String,
    pub comment: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectReaction {
    pub user_id: String,
    pub reaction: ReactionType,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReactionType {
    Upvote,
    Downvote,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectLocation {
    pub province: String,
    pub district: String,
    pub municipality: String,
    pub ward: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub site_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectPersonnel {
    pub name: String,
    pub designation: String,
    pub organization: String,
    pub category: PersonnelCategory,
    pub role: String, // Initiator, Oversight, Supervisor
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FinancialFlow {
    pub amount: f64,
    pub fiscal_year: String,
    pub source: String, // National Budget, Aid, Private
    pub flow_type: String, // Capital, Operational, etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metric {
    pub indicator: String,
    pub target: f64,
    pub actual: f64,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommunicationLog {
    pub action: String,
    pub actor_name: String,
    pub timestamp: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Pipeline,
    Ongoing,
    Completed,
    Delayed,
    Halted,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ProjectTier {
    Federal,
    Provincial,
    Local,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PersonnelCategory {
    Official,
    Contractor,
    Political,
    Donor,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    pub project_code: String,
    pub title: String,
    pub description: String,
    pub tier_level: ProjectTier,
    pub sector: String,
    pub implementation_modality: String,
    pub estimated_total_cost: f64,
    pub province: String,
    pub district: String,
    pub municipality: String,
    pub ward: i32,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubmitOpinionRequest {
    pub comment: String,
}

#[derive(Debug, Deserialize)]
pub struct ReactRequest {
    pub reaction: ReactionType,
}
