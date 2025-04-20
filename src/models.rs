use std::sync::Arc;
use crate::registry::PluginRegistry;

/// Shared app state for all requests
#[derive(Clone)]
pub struct AppState {
    pub registry: Arc<PluginRegistry>,
}

/// Request format for POST /execute
#[derive(serde::Deserialize)]
pub struct PluginExecutionRequest {
    pub plugin: String,
    pub payload: Option<serde_json::Value>,

    /// 🔐 List of scopes the agent has
    pub scopes: Vec<String>, // e.g. ["plugin:echo", "plugin:time"]
}

/// Standard plugin response format
#[derive(serde::Serialize)]
pub struct PluginExecutionResponse {
    pub output: String,
}
