use crate::state::SharedState;
use crate::error::AppError;
use super::model::{Project, CreateProjectRequest};
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
}
