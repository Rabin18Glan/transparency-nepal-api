use super::controller;
use crate::core::state::SharedState;
use axum::{routing::get, Router};

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(controller::ws_handler))
}
