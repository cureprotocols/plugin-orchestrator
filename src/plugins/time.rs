use async_trait::async_trait;
use chrono::Utc;
use serde_json::Value;
use super::PluginExecutor;

/// TimePlugin — returns the current UTC timestamp
pub struct TimePlugin;

#[async_trait]
impl PluginExecutor for TimePlugin {
    async fn execute(&self, _payload: Option<Value>) -> String {
        match Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true) {
            ts => format!("Current UTC time is: {}", ts),
        }
    }
}
