use axum::Router;
use crate::state::SharedState;

pub mod auth;
pub mod projects;
pub mod contributions;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/projects", projects::routes())
        .nest("/contributions", contributions::routes())
}
