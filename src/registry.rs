use std::collections::HashMap;

use crate::plugins::{self, PluginExecutor};

/// PluginRegistry holds all available plugins keyed by name.
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn PluginExecutor + Send + Sync>>,
}

impl PluginRegistry {
    /// Initializes the registry and loads available plugins
    pub fn new() -> Self {
        let mut plugins: HashMap<String, Box<dyn PluginExecutor + Send + Sync>> = HashMap::new();

        // Register plugins here
        plugins.insert("echo".into(), Box::new(plugins::echo::EchoPlugin));
        plugins.insert("time".into(), Box::new(plugins::time::TimePlugin));

        PluginRegistry { plugins }
    }

    /// Retrieves a plugin by name (if registered)
    pub fn get(&self, name: &str) -> Option<&Box<dyn PluginExecutor + Send + Sync>> {
        self.plugins.get(name)
    }

    /// Lists all available plugin names
    #[allow(dead_code)]
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}
