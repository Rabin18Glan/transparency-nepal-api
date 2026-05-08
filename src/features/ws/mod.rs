use crate::core::state::SharedState;
use axum::Router;

pub mod controller;
pub mod routes;

pub fn routes() -> Router<SharedState> {
    routes::routes()
}
