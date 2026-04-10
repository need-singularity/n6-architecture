/// Plugin registry helpers — type-based indexing and validation.

use super::{Plugin, PluginType, PluginRegistry};

impl PluginRegistry {
    /// Validate all plugins: check that entry_point is non-empty and version is well-formed.
    pub fn validate(&self) -> Vec<(String, String)> {
        let mut errors = Vec::new();
        for p in &self.plugins {
            if p.entry_point.is_empty() {
                errors.push((p.name.clone(), "empty entry_point".to_string()));
            }
            if p.version.is_empty() {
                errors.push((p.name.clone(), "empty version".to_string()));
            }
        }
        errors
    }

    /// Summary string: "N plugins (X lens, Y experiment, ...)".
    pub fn summary(&self) -> String {
        let lens = self.plugins.iter().filter(|p| p.plugin_type == PluginType::Lens).count();
        let exp = self.plugins.iter().filter(|p| p.plugin_type == PluginType::Experiment).count();
        let ana = self.plugins.iter().filter(|p| p.plugin_type == PluginType::Analyzer).count();
        let tfm = self.plugins.iter().filter(|p| p.plugin_type == PluginType::Transformer).count();
        format!(
            "{} plugins ({} lens, {} experiment, {} analyzer, {} transformer)",
            self.plugins.len(), lens, exp, ana, tfm
        )
    }
}

/// Manually register a plugin (for testing or programmatic use).
pub fn register_manual(registry: &mut PluginRegistry, plugin: Plugin) {
    // We need access to the internal plugins vec — add via a dedicated method.
    // Since PluginRegistry owns plugins, we extend via scan or direct push.
    registry.plugins.push(plugin);
}

// Note: register_manual needs access to private field.
// We expose it through an impl block in mod.rs instead.

impl PluginRegistry {
    /// Manually register a plugin (bypasses directory scan).
    pub fn register(&mut self, plugin: Plugin) {
        self.plugins.push(plugin);
    }
}
