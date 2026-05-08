use super::controller;
use crate::core::state::SharedState;
use axum::{routing::post, Router};

pub fn routes() -> Router<SharedState> {
    Router::new().route(
        "/",
        post(controller::create_contribution).get(controller::list_contributions),
    )
}
