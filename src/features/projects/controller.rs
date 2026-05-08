use super::model::{
    CreateProjectRequest, Project, ReactRequest, SubmitOpinionRequest, UpdateProjectRequest,
};
use super::service::ProjectService;
use crate::core::error::AppError;
use crate::core::state::SharedState;
use crate::shared::auth::AuthUser;
use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

pub async fn create_project(
    State(state): State<SharedState>,
    _user: AuthUser,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let service = ProjectService::new(state);
    let result = service.create_project(payload).await?;

    Ok(Json(result))
}

pub async fn update_project(
    State(state): State<SharedState>,
    _user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let service = ProjectService::new(state);
    let result = service.update_project(id, payload).await?;

    Ok(Json(result))
}

pub async fn get_projects(
    State(state): State<SharedState>,
) -> Result<Json<Vec<Project>>, AppError> {
    let service = ProjectService::new(state);
    let projects = service.list_projects().await?;

    Ok(Json(projects))
}

pub async fn get_project_detail(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<Project>, AppError> {
    let service = ProjectService::new(state);
    let project = service.get_project(id).await?;

    Ok(Json(project))
}

pub async fn submit_opinion(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(payload): Json<SubmitOpinionRequest>,
) -> Result<Json<Project>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let service = ProjectService::new(state);
    let updated_project = service.add_opinion(id, payload.comment).await?;

    Ok(Json(updated_project))
}

pub async fn react(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(payload): Json<ReactRequest>,
) -> Result<Json<Project>, AppError> {
    let service = ProjectService::new(state);
    let updated_project = service.react(id, payload.reaction).await?;

    Ok(Json(updated_project))
}
