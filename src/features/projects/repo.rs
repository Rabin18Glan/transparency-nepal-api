use crate::error::AppError;
use crate::state::SharedState;
use super::model::{Project, CreateProjectRequest, ProjectStatus, ProjectLocation, ProjectOpinion, ReactionType};
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
            project_code: payload.project_code,
            title: payload.title,
            description: payload.description,
            tier_level: payload.tier_level,
            sector: payload.sector,
            implementation_modality: payload.implementation_modality,
            status: ProjectStatus::Pipeline,
            planned_start_date: None,
            estimated_completion_date: None,
            estimated_total_cost: payload.estimated_total_cost,
            location: ProjectLocation {
                province: payload.province,
                district: payload.district,
                municipality: payload.municipality,
                ward: payload.ward,
                latitude: payload.latitude,
                longitude: payload.longitude,
                site_description: None,
            },
            personnel: vec![],
            financial_flows: vec![],
            results_framework: vec![],
            communications_log: vec![],
            opinions: vec![],
            reactions: vec![] ,
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

    pub async fn get_project_by_id(&self, id: String) -> Result<Project, AppError> {
        let clean_id = id.replace("project:", "");
        let project: Option<Project> = self.state.db
            .select(("project", &clean_id))
            .await
            .map_err(|e| {
                tracing::error!("SURREALDB SELECT ERROR for {}: {}", clean_id, e);
                AppError::InternalServerError(format!("Database error: {}", e))
            })?;

        project.ok_or_else(|| AppError::NotFound(format!("Project with ID {} not found", id)))
    }

    pub async fn add_opinion(&self, project_id: String, opinion: ProjectOpinion) -> Result<Project, AppError> {
        let mut project = self.get_project_by_id(project_id.clone()).await?;
        project.opinions.push(opinion);

        let clean_id = project_id.replace("project:", "");
        let mut update_content = project.clone();
        update_content.id = None; // Avoid SurrealDB conflict with explicit ID in body

        let updated: Option<Project> = self.state.db
            .update(("project", &clean_id))
            .content(update_content)
            .await
            .map_err(|e| {
                tracing::error!("SURREALDB UPDATE ERROR (opinion) for {}: {}", clean_id, e);
                AppError::InternalServerError(format!("Database error: {}", e))
            })?;

        updated.ok_or_else(|| AppError::NotFound(format!("Project with ID {} not found", project_id)))
    }

    pub async fn react(&self, project_id: String, user_id: String, reaction_type: ReactionType) -> Result<Project, AppError> {
        let mut project = self.get_project_by_id(project_id.clone()).await?;
        
        // Find existing reaction from this user and update it, or add new
        if let Some(existing) = project.reactions.iter_mut().find(|r| r.user_id == user_id) {
            existing.reaction = reaction_type;
            existing.timestamp = Utc::now().to_rfc3339();
        } else {
            project.reactions.push(super::model::ProjectReaction {
                user_id,
                reaction: reaction_type,
                timestamp: Utc::now().to_rfc3339(),
            });
        }

        let clean_id = project_id.replace("project:", "");
        let mut update_content = project.clone();
        update_content.id = None; // Avoid SurrealDB conflict with explicit ID in body

        let updated: Option<Project> = self.state.db
            .update(("project", &clean_id))
            .content(update_content)
            .await
            .map_err(|e| {
                tracing::error!("SURREALDB UPDATE ERROR (react) for {}: {}", clean_id, e);
                AppError::InternalServerError(format!("Database error: {}", e))
            })?;

        updated.ok_or_else(|| AppError::NotFound(format!("Project with ID {} not found", project_id)))
    }
}
