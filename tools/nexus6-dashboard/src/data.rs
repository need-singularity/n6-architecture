use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn nexus6_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/ghost".to_string());
    PathBuf::from(home).join(".nexus6")
}

pub fn repo_root() -> PathBuf {
    if let Ok(val) = std::env::var("N6_REPO_ROOT") {
        return PathBuf::from(val);
    }
    PathBuf::from("/Users/ghost/Dev/n6-architecture")
}

pub fn read_json_file(filename: &str) -> Result<Value, String> {
    let path = nexus6_dir().join(filename);
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("read {}: {}", path.display(), e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("parse {}: {}", path.display(), e))
}

pub fn read_growth_log() -> Vec<Value> {
    let path = repo_root()
        .join("tools/nexus6/scripts/growth_log.jsonl");
    let file = match fs::File::open(&path) {
        Ok(f) => f,
        Err(_) => return vec![],
    };
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str(&line).ok())
        .collect()
}

pub fn read_daemon_log_lines(max_lines: usize) -> Vec<String> {
    let path = nexus6_dir().join("growth.log");
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let start = if lines.len() > max_lines { lines.len() - max_lines } else { 0 };
    lines[start..].to_vec()
}

pub fn read_benchmark() -> Result<Value, String> {
    read_json_file("benchmark-results.json")
}

pub fn read_engine_fitness() -> Result<Value, String> {
    read_json_file("engine_fitness.json")
}
