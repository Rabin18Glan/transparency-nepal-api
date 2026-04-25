use axum::{routing::post, Router};
use crate::state::SharedState;

pub mod controller;
pub mod service;
pub mod model;
pub mod repo;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/otp/request", post(controller::request_otp))
        .route("/otp/verify", post(controller::verify_otp))
}
