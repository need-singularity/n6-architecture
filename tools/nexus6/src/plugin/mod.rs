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
