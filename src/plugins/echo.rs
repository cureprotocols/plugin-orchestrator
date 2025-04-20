use async_trait::async_trait;
use serde_json::Value;
use super::PluginExecutor;

/// EchoPlugin — echoes back the provided message field
pub struct EchoPlugin;

#[async_trait]
impl PluginExecutor for EchoPlugin {
    async fn execute(&self, payload: Option<Value>) -> String {
        match payload {
            Some(Value::Object(map)) => {
                if let Some(Value::String(message)) = map.get("message") {
                    return format!("You said: {}", message);
                }
                "Missing 'message' key in object.".to_string()
            }
            Some(_) => "Invalid payload format — expected an object.".to_string(),
            None => "No payload provided.".to_string(),
        }
    }
}

