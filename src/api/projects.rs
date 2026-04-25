use axum::{routing::{post, get}, Router};
use crate::state::SharedState;
use crate::features::projects;

pub fn routes() -> Router<SharedState> {
    Router::new()
        // POST /api/projects
        .route("/", post(projects::controller::create_project))
        // GET /api/projects
        .route("/", get(projects::controller::get_projects))
}
