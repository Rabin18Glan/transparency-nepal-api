use super::controller;
use crate::core::state::SharedState;
use axum::{routing::post, Router};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/otp/request", post(controller::request_otp))
        .route("/otp/verify", post(controller::verify_otp))
}
