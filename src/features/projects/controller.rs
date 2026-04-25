use axum::{extract::State, Json};
use crate::state::SharedState;
use crate::error::AppError;
use super::model::{CreateProjectRequest, Project};
use super::service::ProjectService;
use validator::Validate;

pub async fn create_project(
    State(state): State<SharedState>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let service = ProjectService::new(state);
    let result = service.create_project(payload).await?;

    Ok(Json(result))
}

pub async fn get_projects(
    State(state): State<SharedState>,
) -> Result<Json<Vec<Project>>, AppError> {
    let service = ProjectService::new(state);
    let projects = service.list_projects().await?;
    
    Ok(Json(projects))
}
