use super::controller;
use crate::core::state::SharedState;
use axum::{routing::get, Router};

pub fn routes() -> Router<SharedState> {
    Router::new().route(
        "/",
        get(controller::list_donations).post(controller::create_donation),
    )
}
