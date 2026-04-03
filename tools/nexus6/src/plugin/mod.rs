//! Plugin system for extending NEXUS-6 with external modules.
/// Plugin system — discover and manage NEXUS-6 plugins via TOML manifests.
pub mod registry;
pub mod loader;

/// A loaded plugin descriptor.
#[derive(Debug, Clone, PartialEq)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub plugin_type: PluginType,
    pub entry_point: String,
}

/// Classification of plugin capabilities.
#[derive(Debug, Clone, PartialEq)]
pub enum PluginType {
    Lens,
    Experiment,
    Analyzer,
    Transformer,
}

impl PluginType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "lens" => Some(Self::Lens),
            "experiment" => Some(Self::Experiment),
            "analyzer" => Some(Self::Analyzer),
            "transformer" => Some(Self::Transformer),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lens => "lens",
            Self::Experiment => "experiment",
            Self::Analyzer => "analyzer",
            Self::Transformer => "transformer",
        }
    }
}

/// Registry that scans a directory for plugin manifests and provides lookup.
///
#[cfg(test)]
mod tests {
    use super::*;

    fn make_plugin(name: &str, ptype: PluginType) -> Plugin {
        Plugin {
            name: name.into(),
            version: "1.0.0".into(),
            plugin_type: ptype,
            entry_point: format!("{}.rs", name),
        }
    }

    #[test]
    fn test_plugin_type_from_str_roundtrip() {
        let types = [PluginType::Lens, PluginType::Experiment, PluginType::Analyzer, PluginType::Transformer];
        for pt in &types {
            let s = pt.as_str();
            let parsed = PluginType::from_str(s);
            assert_eq!(parsed.as_ref(), Some(pt));
        }
        assert_eq!(PluginType::from_str("unknown"), None);
    }

    #[test]
    fn test_plugin_registry_register_and_lookup() {
        let mut registry = PluginRegistry::new("/tmp/nexus6_plugins_test");
        assert!(registry.is_empty());

        // Register 6 plugins (n=6)
        for i in 0..6 {
            registry.register(make_plugin(
                &format!("plugin_{}", i),
                if i < 3 { PluginType::Lens } else { PluginType::Experiment },
            ));
        }
        assert_eq!(registry.len(), 6);

        let p = registry.load("plugin_0");
        assert!(p.is_some());
        assert_eq!(p.unwrap().plugin_type, PluginType::Lens);

        assert!(registry.load("nonexistent").is_none());
    }

    #[test]
    fn test_plugin_registry_list_by_type() {
        let mut registry = PluginRegistry::new("/tmp/nexus6_plugins_test2");
        registry.register(make_plugin("lens_a", PluginType::Lens));
        registry.register(make_plugin("lens_b", PluginType::Lens));
        registry.register(make_plugin("exp_a", PluginType::Experiment));
        registry.register(make_plugin("ana_a", PluginType::Analyzer));

        let lenses = registry.list_by_type(&PluginType::Lens);
        assert_eq!(lenses.len(), 2);
        let analyzers = registry.list_by_type(&PluginType::Analyzer);
        assert_eq!(analyzers.len(), 1);
        let transformers = registry.list_by_type(&PluginType::Transformer);
        assert_eq!(transformers.len(), 0);
    }

    #[test]
    fn test_plugin_registry_validate() {
        let mut registry = PluginRegistry::new("/tmp/nexus6_plugins_test3");
        registry.register(make_plugin("good_plugin", PluginType::Lens));
        registry.register(Plugin {
            name: "bad_plugin".into(),
            version: "".into(), // empty version
            plugin_type: PluginType::Analyzer,
            entry_point: "bad.rs".into(),
        });

        let errors = registry.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].0, "bad_plugin");
    }

    #[test]
    fn test_plugin_registry_summary() {
        let mut registry = PluginRegistry::new("/tmp/nexus6_plugins_test4");
        registry.register(make_plugin("l1", PluginType::Lens));
        registry.register(make_plugin("e1", PluginType::Experiment));
        registry.register(make_plugin("a1", PluginType::Analyzer));
        registry.register(make_plugin("t1", PluginType::Transformer));

        let summary = registry.summary();
        assert!(summary.contains("4 plugins"));
        assert!(summary.contains("1 lens"));
        assert!(summary.contains("1 experiment"));
    }

    #[test]
    fn test_parse_manifest() {
        let content = r#"
# NEXUS-6 plugin manifest
name = "sigma_lens"
version = "0.6.0"
type = "lens"
entry_point = "sigma_lens.rs"
"#;
        let plugin = loader::parse_manifest(content);
        assert!(plugin.is_some());
        let p = plugin.unwrap();
        assert_eq!(p.name, "sigma_lens");
        assert_eq!(p.version, "0.6.0");
        assert_eq!(p.plugin_type, PluginType::Lens);
        assert_eq!(p.entry_point, "sigma_lens.rs");
    }

    #[test]
    fn test_parse_manifest_missing_required_field() {
        // Missing "type" field
        let content = r#"
name = "broken"
version = "1.0"
entry_point = "broken.rs"
"#;
        assert!(loader::parse_manifest(content).is_none());
    }

    #[test]
    fn test_parse_manifest_quoted_values() {
        let content = r#"
name = 'n6_plugin'
version = "0.1.0"
type = 'transformer'
entry_point = "n6_plugin.rs"
"#;
        let plugin = loader::parse_manifest(content).unwrap();
        assert_eq!(plugin.name, "n6_plugin");
        assert_eq!(plugin.plugin_type, PluginType::Transformer);
    }
}

pub struct PluginRegistry {
    plugins: Vec<Plugin>,
    plugin_dir: String,
}

impl PluginRegistry {
    /// Create a new registry pointing at the given plugin directory.
    pub fn new(dir: &str) -> Self {
        Self {
            plugins: Vec::new(),
            plugin_dir: dir.to_string(),
        }
    }

    /// Scan the plugin directory for `.toml` manifest files and load them.
    ///
    /// Manifest format (minimal TOML subset, hand-parsed):
    /// ```toml
    /// name = "my_plugin"
    /// version = "0.1.0"
    /// type = "lens"
    /// entry_point = "my_plugin.rs"
    /// ```
    pub fn scan_plugins(&mut self) {
        self.plugins.clear();

        let dir = match std::fs::read_dir(&self.plugin_dir) {
            Ok(d) => d,
            Err(_) => return, // directory doesn't exist — no plugins
        };

        for entry in dir.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                if let Some(plugin) = loader::load_manifest(&path) {
                    self.plugins.push(plugin);
                }
            }
        }

        // Deterministic ordering by name
        self.plugins.sort_by(|a, b| a.name.cmp(&b.name));
    }

    /// Look up a plugin by name.
    pub fn load(&self, name: &str) -> Option<&Plugin> {
        self.plugins.iter().find(|p| p.name == name)
    }

    /// Return all registered plugins.
    pub fn list(&self) -> &[Plugin] {
        &self.plugins
    }

    /// Return plugins of a specific type.
    pub fn list_by_type(&self, plugin_type: &PluginType) -> Vec<&Plugin> {
        self.plugins.iter().filter(|p| &p.plugin_type == plugin_type).collect()
    }

    /// Number of loaded plugins.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// The directory being scanned.
    pub fn plugin_dir(&self) -> &str {
        &self.plugin_dir
    }
}
