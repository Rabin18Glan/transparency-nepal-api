use crate::state::SharedState;
use crate::error::AppError;
use super::model::{Project, CreateProjectRequest, ProjectOpinion, ReactionType};
use super::repo::ProjectRepository;

pub struct ProjectService {
    repo: ProjectRepository,
}

impl ProjectService {
    pub fn new(state: SharedState) -> Self {
        Self {
            repo: ProjectRepository::new(state),
        }
    }

    pub async fn create_project(&self, payload: CreateProjectRequest) -> Result<Project, AppError> {
        tracing::info!("Creating new project: {}", payload.title);
        let project = self.repo.create_project(payload).await?;
        tracing::info!("Successfully created project ID: {:?}", project.id);
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

    pub async fn add_opinion(&self, id: String, comment: String) -> Result<Project, AppError> {
        tracing::info!("Adding opinion to project ID: {}", id);
        let opinion = ProjectOpinion {
            user_id: "user_1".to_string(),
            user_name: "Citizen X".to_string(),
            comment,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.repo.add_opinion(id, opinion).await
    }

    pub async fn react(&self, id: String, reaction: ReactionType) -> Result<Project, AppError> {
        tracing::info!("Adding reaction to project ID: {}", id);
        let user_id = "user_1".to_string(); // In real app, this would come from session
        self.repo.react(id, user_id, reaction).await
    }
}
