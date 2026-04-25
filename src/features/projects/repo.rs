use crate::error::AppError;
use crate::state::SharedState;
use super::model::{Project, CreateProjectRequest, ProjectStatus};
use chrono::Utc;

pub struct ProjectRepository {
    state: SharedState,
}

impl ProjectRepository {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    pub async fn create_project(&self, payload: CreateProjectRequest) -> Result<Project, AppError> {
        let record = Project {
            id: None,
            title: payload.title,
            description: payload.description,
            expected_budget: payload.expected_budget,
            status: ProjectStatus::Planning,
            latitude: payload.latitude,
            longitude: payload.longitude,
            created_at: Utc::now().to_rfc3339(),
        };

        let created: Option<Project> = self.state.db
            .create("project")
            .content(record)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

        created.ok_or_else(|| AppError::InternalServerError("Failed to return created project".to_string()))
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, AppError> {
        let projects: Vec<Project> = self.state.db
            .select("project")
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

        Ok(projects)
    }
}
