use async_trait::async_trait;
use serde_json::Value;

/// Trait that all plugins must implement
#[async_trait]
pub trait PluginExecutor {
    /// Executes the plugin with optional JSON payload
    async fn execute(&self, payload: Option<Value>) -> String;
}

// Register plugins here
pub mod echo;
pub mod time;
