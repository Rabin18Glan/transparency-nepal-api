pub mod model;
pub mod repo;
pub mod service;
pub mod controller;

use axum::Router;
use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    controller::routes()
}
