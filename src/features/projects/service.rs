use super::model::{CreateProjectRequest, Project, ProjectOpinion, ReactionType};
use super::repo::ProjectRepository;
use crate::core::error::AppError;
use crate::core::state::SharedState;

pub struct ProjectService {
    repo: ProjectRepository,
    state: SharedState,
}

impl ProjectService {
    pub fn new(state: SharedState) -> Self {
        Self {
            repo: ProjectRepository::new(state.clone()),
            state,
        }
    }

    pub async fn create_project(&self, payload: CreateProjectRequest) -> Result<Project, AppError> {
        tracing::info!("Creating new project: {}", payload.title);
        let project = self.repo.create_project(payload).await?;
        tracing::info!("Successfully created project ID: {:?}", project.id);

        // Broadcast to WS
        let event = serde_json::json!({
            "type": "project_created",
            "data": project
        });
        let _ = self.state.tx.send(event.to_string());

        Ok(project)
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>, AppError> {
        tracing::info!("Retrieving all active projects for Map View");
        let projects = self.repo.get_all_projects().await?;
        Ok(projects)
    }

    pub async fn get_project(&self, id: String) -> Result<Project, AppError> {
        tracing::info!("Retrieving project details for ID: {}", id);
        self.repo.get_project_by_id(id).await
    }

    pub async fn update_project(
        &self,
        id: String,
        payload: super::model::UpdateProjectRequest,
    ) -> Result<Project, AppError> {
        tracing::info!("Updating project ID: {}", id);
        let project = self.repo.update_project(id, payload).await?;

        // Broadcast to WS
        let event = serde_json::json!({
            "type": "project_updated",
            "data": project
        });
        let _ = self.state.tx.send(event.to_string());

        Ok(project)
    }

    pub async fn add_opinion(&self, id: String, comment: String) -> Result<Project, AppError> {
        tracing::info!("Adding opinion to project ID: {}", id);
        let opinion = ProjectOpinion {
            user_id: "user_1".to_string(),
            user_name: "Citizen X".to_string(),
            comment,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let project = self.repo.add_opinion(id, opinion).await?;

        // Broadcast to WS
        let event = serde_json::json!({
            "type": "project_opinion_added",
            "data": project
        });
        let _ = self.state.tx.send(event.to_string());

        Ok(project)
    }

    pub async fn react(&self, id: String, reaction: ReactionType) -> Result<Project, AppError> {
        tracing::info!("Adding reaction to project ID: {}", id);
        let user_id = "user_1".to_string(); // In real app, this would come from session
        let project = self.repo.react(id, user_id, reaction).await?;

        // Broadcast to WS
        let event = serde_json::json!({
            "type": "project_reaction_updated",
            "data": project
        });
        let _ = self.state.tx.send(event.to_string());

        Ok(project)
    }
}
