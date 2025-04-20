use axum::{routing::post, Router};
use std::{net::SocketAddr, sync::Arc};
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod orchestrator;
mod registry;
mod models;
mod plugins;

use orchestrator::execute_plugin;
use registry::PluginRegistry;
use models::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build the plugin registry and state
    let registry = PluginRegistry::new();
    let state = AppState {
        registry: Arc::new(registry),
    };

    // Set up routes
    let app = Router::new()
        .route("/execute", post(execute_plugin))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    tracing::info!("?? Plugin Orchestrator running at http://{}/", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
