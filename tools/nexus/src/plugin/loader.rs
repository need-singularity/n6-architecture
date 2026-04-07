/// Plugin manifest loader — hand-parses minimal TOML without external crates.

use std::path::Path;
use super::{Plugin, PluginType};

/// Load a plugin manifest from a `.toml` file.
///
/// Expected keys: `name`, `version`, `type`, `entry_point`.
/// Lines starting with `#` are comments. Whitespace is trimmed.
pub fn load_manifest(path: &Path) -> Option<Plugin> {
    let content = std::fs::read_to_string(path).ok()?;
    parse_manifest(&content)
}

/// Parse manifest content (for testability without filesystem).
pub fn parse_manifest(content: &str) -> Option<Plugin> {
    let mut name: Option<String> = None;
    let mut version: Option<String> = None;
    let mut plugin_type: Option<PluginType> = None;
    let mut entry_point: Option<String> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim();
            let val = line[pos + 1..].trim();
            // Strip quotes
            let val = val.trim_matches('"').trim_matches('\'');
            match key {
                "name" => name = Some(val.to_string()),
                "version" => version = Some(val.to_string()),
                "type" => plugin_type = PluginType::from_str(val),
                "entry_point" => entry_point = Some(val.to_string()),
                _ => {} // ignore unknown keys
            }
        }
    }

    Some(Plugin {
        name: name?,
        version: version.unwrap_or_else(|| "0.0.0".to_string()),
        plugin_type: plugin_type?,
        entry_point: entry_point?,
    })
}
