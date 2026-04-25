pub mod model;
pub mod repo;
pub mod service;
pub mod controller;

use axum::{routing::{post, get}, Router};
use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(controller::create_project))
        .route("/", get(controller::get_projects))
}

