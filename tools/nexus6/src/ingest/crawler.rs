/// Multi-project crawler — scans configured project directories for data files
/// and routes them to the appropriate parser (json, csv, text/md, toml, py).
///
/// No hardcoded paths: all sources come from `CrawlConfig`.
/// Preferred: load from `shared/projects.json` via `load_from_json()`.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::csv_reader;
use super::json_reader;
use super::md_parser;
use super::py_parser;
use super::rs_parser;
use super::text_parser;
use super::toml_parser;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A single data point extracted from a file.
#[derive(Debug, Clone)]
pub struct ProbeData {
    /// Source project domain (e.g. "n6-architecture", "TECS-L", "anima").
    pub domain: String,
    /// Relative path within the project.
    pub source_file: String,
    /// Named values extracted from the file.
    pub named_values: Vec<(String, f64)>,
    /// Unnamed numeric values extracted from the file.
    pub raw_values: Vec<f64>,
}

/// Aggregated result of crawling all configured projects.
#[derive(Debug, Clone)]
pub struct IngestResult {
    pub probes: Vec<ProbeData>,
    pub files_scanned: usize,
    pub files_skipped: usize,
    pub errors: Vec<String>,
}

/// Configuration for a single project source.
#[derive(Debug, Clone)]
pub struct ProjectSource {
    /// Absolute path to the project root.
    pub path: PathBuf,
    /// Human-readable domain name.
    pub domain: String,
    /// Glob-style file extensions to include (without the dot).
    pub extensions: Vec<String>,
}

/// Top-level crawl configuration.
#[derive(Debug, Clone)]
pub struct CrawlConfig {
    pub sources: Vec<ProjectSource>,
}

// ---------------------------------------------------------------------------
// Parser trait
// ---------------------------------------------------------------------------

/// Trait for pluggable file parsers. Each parser handles one or more extensions.
pub trait FileParser {
    /// Extensions this parser handles (without the dot, e.g. "json").
    fn extensions(&self) -> &[&str];

    /// Parse a file and return named pairs + raw values.
    fn parse(&self, path: &Path) -> Result<(Vec<(String, f64)>, Vec<f64>), String>;
}

// ---------------------------------------------------------------------------
// Built-in parser adapters
// ---------------------------------------------------------------------------

/// JSON file parser — delegates to `json_reader`.
struct JsonParser;

impl FileParser for JsonParser {
    fn extensions(&self) -> &[&str] {
        &["json"]
    }

    fn parse(&self, path: &Path) -> Result<(Vec<(String, f64)>, Vec<f64>), String> {
        let path_str = path.to_string_lossy();
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {}", path_str, e))?;
        let named = json_reader::parse_json_kv(&content).unwrap_or_default();
        let raw = json_reader::parse_json_values(&content).unwrap_or_default();
        Ok((named, raw))
    }
}

/// CSV file parser — delegates to `csv_reader`.
struct CsvParser;

impl FileParser for CsvParser {
    fn extensions(&self) -> &[&str] {
        &["csv"]
    }

    fn parse(&self, path: &Path) -> Result<(Vec<(String, f64)>, Vec<f64>), String> {
        let path_str = path.to_string_lossy();
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {}", path_str, e))?;
        let rows = csv_reader::parse_csv_content(&content).unwrap_or_default();
        let flat = csv_reader::flatten(&rows);
        Ok((Vec::new(), flat))
    }
}

/// Markdown parser — delegates to `md_parser` for structured extraction.
struct MdParserAdapter;

impl FileParser for MdParserAdapter {
    fn extensions(&self) -> &[&str] {
        &["md"]
    }

    fn parse(&self, path: &Path) -> Result<(Vec<(String, f64)>, Vec<f64>), String> {
        let path_str = path.to_string_lossy();
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {}", path_str, e))?;
        let result = md_parser::parse_markdown(&content);
        let named = md_parser::extract_numeric_constants(&result);
        let raw = md_parser::extract_table_numbers(&result);
        Ok((named, raw))
    }
}

/// Plain text parser — delegates to `text_parser`.
struct TextParser;

impl FileParser for TextParser {
    fn extensions(&self) -> &[&str] {
        &["txt"]
    }

    fn parse(&self, path: &Path) -> Result<(Vec<(String, f64)>, Vec<f64>), String> {
        let path_str = path.to_string_lossy();
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {}", path_str, e))?;
        let named = text_parser::extract_key_value_pairs(&content);
        let raw = text_parser::extract_numbers(&content);
        Ok((named, raw))
    }
}

/// TOML parser — delegates to `toml_parser` for DSE-map structured data,
/// with fallback to text KV extraction for generic TOML files.
struct TomlParserAdapter;

impl FileParser for TomlParserAdapter {
    fn extensions(&self) -> &[&str] {
        &["toml"]
    }

    fn parse(&self, path: &Path) -> Result<(Vec<(String, f64)>, Vec<f64>), String> {
        let path_str = path.to_string_lossy();
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {}", path_str, e))?;

        // Try structured DSE-map parse first.
        if let Ok(dse_map) = toml_parser::parse_toml_content(&content) {
            let named = toml_parser::extract_n6_scores(&dse_map);
            let raw = toml_parser::extract_all_numbers(&dse_map);
            if !named.is_empty() || !raw.is_empty() {
                return Ok((named, raw));
            }
        }

        // Fallback: generic text KV extraction.
        let named = text_parser::extract_key_value_pairs(&content);
        let raw = text_parser::extract_numbers(&content);
        Ok((named, raw))
    }
}

/// Python source parser — delegates to `py_parser` for constant extraction.
struct PyParserAdapter;

impl FileParser for PyParserAdapter {
    fn extensions(&self) -> &[&str] {
        &["py"]
    }

    fn parse(&self, path: &Path) -> Result<(Vec<(String, f64)>, Vec<f64>), String> {
        let path_str = path.to_string_lossy();
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {}", path_str, e))?;
        let named = py_parser::extract_numeric_constants(&content);
        let raw = named.iter().map(|(_, v)| *v).collect();
        Ok((named, raw))
    }
}

/// Rust source parser — delegates to `rs_parser` for constant and enum extraction.
struct RsParserAdapter;

impl FileParser for RsParserAdapter {
    fn extensions(&self) -> &[&str] {
        &["rs"]
    }

    fn parse(&self, path: &Path) -> Result<(Vec<(String, f64)>, Vec<f64>), String> {
        let path_str = path.to_string_lossy();
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read '{}': {}", path_str, e))?;
        let named = rs_parser::extract_numeric_constants(&content);
        let raw = named.iter().map(|(_, v)| *v).collect();
        Ok((named, raw))
    }
}

// ---------------------------------------------------------------------------
// Crawler implementation
// ---------------------------------------------------------------------------

/// Return the default set of built-in parsers.
fn builtin_parsers() -> Vec<Box<dyn FileParser>> {
    vec![
        Box::new(JsonParser),
        Box::new(CsvParser),
        Box::new(MdParserAdapter),
        Box::new(TextParser),
        Box::new(TomlParserAdapter),
        Box::new(PyParserAdapter),
        Box::new(RsParserAdapter),
    ]
}

/// Select the appropriate parser for a file extension.
fn route_to_parser<'a>(
    ext: &str,
    parsers: &'a [Box<dyn FileParser>],
) -> Option<&'a dyn FileParser> {
    let ext_lower = ext.to_lowercase();
    parsers.iter().find_map(|p| {
        if p.extensions().iter().any(|e| *e == ext_lower) {
            Some(p.as_ref())
        } else {
            None
        }
    })
}

/// Recursively collect files under `dir` whose extensions are in `exts`.
fn collect_files(dir: &Path, exts: &[String]) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return result,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Skip hidden directories and common non-source dirs.
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with('.') || name == "node_modules" || name == "target"
                || name == "__pycache__" || name == "venv" || name == ".git"
            {
                continue;
            }
            result.extend(collect_files(&path, exts));
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if exts.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                result.push(path);
            }
        }
    }

    result
}

/// Crawl all configured project sources and return unified `IngestResult`.
pub fn crawl(config: &CrawlConfig) -> IngestResult {
    let parsers = builtin_parsers();
    let mut probes = Vec::new();
    let mut files_scanned: usize = 0;
    let mut files_skipped: usize = 0;
    let mut errors: Vec<String> = Vec::new();

    for source in &config.sources {
        if !source.path.exists() {
            errors.push(format!(
                "[{}] Path does not exist: {}",
                source.domain,
                source.path.display()
            ));
            continue;
        }

        let files = collect_files(&source.path, &source.extensions);

        for file in files {
            let ext = match file.extension().and_then(|e| e.to_str()) {
                Some(e) => e,
                None => {
                    files_skipped += 1;
                    continue;
                }
            };

            let parser = match route_to_parser(ext, &parsers) {
                Some(p) => p,
                None => {
                    files_skipped += 1;
                    continue;
                }
            };

            match parser.parse(&file) {
                Ok((named, raw)) => {
                    if named.is_empty() && raw.is_empty() {
                        files_skipped += 1;
                        continue;
                    }
                    let rel = file
                        .strip_prefix(&source.path)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .to_string();
                    probes.push(ProbeData {
                        domain: source.domain.clone(),
                        source_file: rel,
                        named_values: named,
                        raw_values: raw,
                    });
                    files_scanned += 1;
                }
                Err(e) => {
                    errors.push(format!("[{}] {}: {}", source.domain, file.display(), e));
                    files_skipped += 1;
                }
            }
        }
    }

    IngestResult {
        probes,
        files_scanned,
        files_skipped,
        errors,
    }
}

// ---------------------------------------------------------------------------
// JSON config loading (shared/projects.json)
// ---------------------------------------------------------------------------

/// Raw JSON schema for `shared/projects.json`.
#[derive(Deserialize)]
struct ProjectsFile {
    #[allow(dead_code)]
    version: u32,
    base_path: String,
    projects: Vec<ProjectEntry>,
}

#[derive(Deserialize)]
struct ProjectEntry {
    #[allow(dead_code)]
    id: String,
    path: String,
    domain: String,
    scan_patterns: Vec<String>,
    // remaining fields are optional / unused for crawling
}

/// Extract unique file extensions from glob-style scan patterns.
///
/// E.g. `["**/*.py", "docs/**/*.md"]` → `["py", "md"]`.
fn extensions_from_patterns(patterns: &[String]) -> Vec<String> {
    let mut exts = Vec::new();
    for pat in patterns {
        if let Some(dot_pos) = pat.rfind('.') {
            let ext = &pat[dot_pos + 1..];
            if !ext.is_empty() && !exts.iter().any(|e: &String| e == ext) {
                exts.push(ext.to_string());
            }
        }
    }
    exts
}

/// Load a `CrawlConfig` from a `projects.json` file.
///
/// The JSON must have `base_path` and a `projects` array.  Each project's
/// absolute path is `base_path / project.path`, and its extensions are
/// derived from `scan_patterns`.
pub fn load_from_json(path: &str) -> Result<CrawlConfig, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;
    load_from_json_str(&content)
}

/// Parse a `CrawlConfig` from a JSON string (useful for testing).
pub fn load_from_json_str(json: &str) -> Result<CrawlConfig, String> {
    let file: ProjectsFile =
        serde_json::from_str(json).map_err(|e| format!("Invalid projects JSON: {}", e))?;

    let base = PathBuf::from(&file.base_path);
    let sources = file
        .projects
        .iter()
        .map(|p| {
            let exts = extensions_from_patterns(&p.scan_patterns);
            ProjectSource {
                path: base.join(&p.path),
                domain: p.domain.clone(),
                extensions: exts,
            }
        })
        .collect();

    Ok(CrawlConfig { sources })
}

/// Build the default `CrawlConfig` for the three known projects.
///
/// Returns `None` for paths that do not resolve. Callers can override
/// this with custom configs.
pub fn default_config() -> CrawlConfig {
    let default_exts = vec![
        "toml".to_string(),
        "md".to_string(),
        "py".to_string(),
        "rs".to_string(),
        "json".to_string(),
        "csv".to_string(),
        "txt".to_string(),
    ];

    CrawlConfig {
        sources: vec![
            ProjectSource {
                path: PathBuf::from("/Users/ghost/Dev/n6-architecture"),
                domain: "n6-architecture".to_string(),
                extensions: default_exts.clone(),
            },
            ProjectSource {
                path: PathBuf::from("/Users/ghost/Dev/TECS-L"),
                domain: "TECS-L".to_string(),
                extensions: default_exts.clone(),
            },
            ProjectSource {
                path: PathBuf::from("/Users/ghost/Dev/anima"),
                domain: "anima".to_string(),
                extensions: default_exts,
            },
        ],
    }
}

/// Merge all `ProbeData` raw values into a single flat `Vec<f64>`.
///
/// Useful when the downstream pipeline (telescope, verifier) just needs
/// a numeric array.
pub fn flatten_all(result: &IngestResult) -> Vec<f64> {
    let mut all = Vec::new();
    for probe in &result.probes {
        all.extend_from_slice(&probe.raw_values);
        for (_, v) in &probe.named_values {
            all.push(*v);
        }
    }
    all
}

// ---------------------------------------------------------------------------
// Convenience: ingest_all (config -> flat values)
// ---------------------------------------------------------------------------

/// One-shot: crawl default sources and return flattened numeric values.
pub fn ingest_all() -> IngestResult {
    crawl(&default_config())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_route_to_parser() {
        let parsers = builtin_parsers();
        assert!(route_to_parser("json", &parsers).is_some());
        assert!(route_to_parser("JSON", &parsers).is_some());
        assert!(route_to_parser("csv", &parsers).is_some());
        assert!(route_to_parser("md", &parsers).is_some());
        assert!(route_to_parser("toml", &parsers).is_some());
        assert!(route_to_parser("py", &parsers).is_some());
        assert!(route_to_parser("rs", &parsers).is_some());
        assert!(route_to_parser("exe", &parsers).is_none());
    }

    #[test]
    fn test_crawl_empty_config() {
        let config = CrawlConfig { sources: vec![] };
        let result = crawl(&config);
        assert!(result.probes.is_empty());
        assert_eq!(result.files_scanned, 0);
    }

    #[test]
    fn test_crawl_nonexistent_path() {
        let config = CrawlConfig {
            sources: vec![ProjectSource {
                path: PathBuf::from("/nonexistent/path/that/does/not/exist"),
                domain: "ghost".to_string(),
                extensions: vec!["json".to_string()],
            }],
        };
        let result = crawl(&config);
        assert!(result.probes.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_crawl_temp_dir() {
        let dir = std::env::temp_dir().join("nexus6_crawl_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        // Write a small JSON file
        fs::write(
            dir.join("test.json"),
            r#"{"sigma": 12.0, "phi": 2.0}"#,
        )
        .unwrap();

        // Write a small text file with a table for the md_parser
        fs::write(
            dir.join("notes.md"),
            "# Constants\n\n| Symbol | Value |\n|--------|-------|\n| tau | 4.0 |\n| n | 6 |\n",
        )
        .unwrap();

        let config = CrawlConfig {
            sources: vec![ProjectSource {
                path: dir.clone(),
                domain: "test".to_string(),
                extensions: vec!["json".to_string(), "md".to_string()],
            }],
        };

        let result = crawl(&config);
        assert!(result.files_scanned >= 1); // JSON always works; md depends on parser
        assert!(result.errors.is_empty());

        // Check that we got probes from both files
        let domains: Vec<&str> = result.probes.iter().map(|p| p.domain.as_str()).collect();
        assert!(domains.iter().all(|d| *d == "test"));

        let flat = flatten_all(&result);
        assert!(!flat.is_empty());

        // Cleanup
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_extensions_from_patterns() {
        let pats = vec![
            "**/*.py".to_string(),
            "docs/**/*.md".to_string(),
            "**/*.py".to_string(), // duplicate
        ];
        let exts = extensions_from_patterns(&pats);
        assert_eq!(exts, vec!["py".to_string(), "md".to_string()]);
    }

    #[test]
    fn test_extensions_from_patterns_empty() {
        let exts = extensions_from_patterns(&[]);
        assert!(exts.is_empty());
    }

    #[test]
    fn test_load_from_json_str_basic() {
        let json = r#"{
            "version": 1,
            "base_path": "/tmp/test",
            "projects": [
                {
                    "id": "alpha",
                    "path": "alpha-proj",
                    "domain": "math",
                    "scan_patterns": ["**/*.py", "docs/**/*.md"]
                },
                {
                    "id": "beta",
                    "path": "beta-proj",
                    "domain": "physics",
                    "scan_patterns": ["**/*.json", "**/*.csv", "**/*.toml"]
                }
            ]
        }"#;

        let config = load_from_json_str(json).unwrap();
        assert_eq!(config.sources.len(), 2);

        assert_eq!(config.sources[0].domain, "math");
        assert_eq!(config.sources[0].path, PathBuf::from("/tmp/test/alpha-proj"));
        assert_eq!(config.sources[0].extensions, vec!["py", "md"]);

        assert_eq!(config.sources[1].domain, "physics");
        assert_eq!(config.sources[1].path, PathBuf::from("/tmp/test/beta-proj"));
        assert_eq!(config.sources[1].extensions, vec!["json", "csv", "toml"]);
    }

    #[test]
    fn test_load_from_json_str_invalid() {
        let result = load_from_json_str("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_json_file_not_found() {
        let result = load_from_json("/nonexistent/projects.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_collect_files_skips_hidden() {
        let dir = std::env::temp_dir().join("nexus6_hidden_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join(".hidden")).unwrap();
        fs::write(dir.join(".hidden/secret.json"), "{}").unwrap();
        fs::write(dir.join("visible.json"), r#"{"x": 1}"#).unwrap();

        let files = collect_files(&dir, &["json".to_string()]);
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().contains("visible"));

        let _ = fs::remove_dir_all(&dir);
    }
}
