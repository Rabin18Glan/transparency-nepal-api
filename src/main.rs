use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod features;
mod common;
mod error;
mod state;
mod config;

#[tokio::main]
async fn main() {
    // 1. Initialize Configuration & Resources (The Source of Truth)
    let state = config::initialize_app_state().await;

    // 2. Initialize Logging (Based on newly loaded config)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&state.config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3. Initialize Routes with Centralized State
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", features::routes())
        .with_state(state.clone());

    // 4. Start Server
    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.server_port));
    tracing::info!("SurrealDB & Redis Ready. Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, starting graceful shutdown");
}
