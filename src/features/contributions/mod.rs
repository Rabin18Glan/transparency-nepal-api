pub mod controller;
pub mod model;
pub mod repo;
pub mod routes;
pub mod service;

use crate::core::state::SharedState;
use axum::Router;

pub fn routes() -> Router<SharedState> {
    routes::routes()
}
