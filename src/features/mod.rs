use crate::core::state::SharedState;
use axum::Router;

pub mod auth;
pub mod contributions;
pub mod donations;
pub mod projects;
pub mod ws;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/projects", projects::routes())
        .nest("/contributions", contributions::routes())
        .nest("/donations", donations::routes())
        .nest("/ws", ws::routes())
}
