use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::models::{AppState, PluginExecutionRequest, PluginExecutionResponse};
use futures::FutureExt;
use std::panic::AssertUnwindSafe;

/// POST /execute
///
/// - Checks scope
/// - Runs plugin safely
pub async fn execute_plugin(
    State(state): State<AppState>,
    Json(payload): Json<PluginExecutionRequest>,
) -> impl IntoResponse {
    let plugin_name = payload.plugin.to_lowercase();
    let required_scope = format!("plugin:{}", plugin_name);

    // 🔐 Scope enforcement
    if !payload.scopes.contains(&required_scope) {
        let error_msg = format!("Access denied: missing scope '{}'", required_scope);
        return (StatusCode::FORBIDDEN, Json(PluginExecutionResponse {
            output: error_msg,
        }))
        .into_response();
    }

    // 🔄 Run plugin safely
    match state.registry.get(&plugin_name) {
        Some(plugin) => {
            let output = match AssertUnwindSafe(plugin.execute(payload.payload))
                .catch_unwind()
                .await
            {
                Ok(output) => output,
                Err(_) => "❌ Plugin crashed unexpectedly.".to_string(),
            };

            let response = PluginExecutionResponse { output };
            (StatusCode::OK, Json(response)).into_response()
        }
        None => {
            let error_msg = format!("Plugin '{}' not found", plugin_name);
            (StatusCode::NOT_FOUND, Json(PluginExecutionResponse { output: error_msg }))
                .into_response()
        }
    }
}

