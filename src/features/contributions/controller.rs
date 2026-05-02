use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::state::SharedState;
use super::model::CreateContributionRequest;
use super::repo::ContributionRepository;
use super::service::ContributionService;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(create_contribution).get(list_contributions))
}

async fn create_contribution(
    State(state): State<SharedState>,
    user: crate::common::auth::AuthUser,
    Json(payload): Json<CreateContributionRequest>,
) -> Json<Value> {
    let repo = Arc::new(ContributionRepository::new(state.clone()));
    let service = ContributionService::new(repo);

    let user_id = format!("user:{}", user.phone_number);

    match service.create_contribution(user_id, payload).await {
        Ok(contribution) => Json(json!({ "status": "success", "data": contribution })),
        Err(e) => Json(json!({ "status": "error", "message": e })),
    }
}

async fn list_contributions(
    State(state): State<SharedState>,
) -> Json<Value> {
    let repo = Arc::new(ContributionRepository::new(state.clone()));
    let service = ContributionService::new(repo);

    match service.list_contributions().await {
        Ok(contributions) => Json(json!({ "status": "success", "data": contributions })),
        Err(e) => Json(json!({ "status": "error", "message": e })),
    }
}
