use super::controller;
use crate::core::state::SharedState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route(
            "/",
            get(controller::get_projects).post(controller::create_project),
        )
        .route(
            "/:id",
            get(controller::get_project_detail).patch(controller::update_project),
        )
        .route("/:id/opinions", post(controller::submit_opinion))
        .route("/:id/react", post(controller::react))
}
