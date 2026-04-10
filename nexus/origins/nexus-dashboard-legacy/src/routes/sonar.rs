use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn discovery_graph_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/ghost".to_string());
    std::path::PathBuf::from(home)
        .join("Dev/nexus/shared/discovery_graph.json")
}

fn count_domains() -> HashMap<String, usize> {
    let path = discovery_graph_path();
    let file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(_) => return HashMap::new(),
    };
    let mut counts: HashMap<String, usize> = HashMap::new();
    for line in BufReader::new(file).lines().flatten() {
        let val: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if val.get("type").and_then(|t| t.as_str()) != Some("node") {
            continue;
        }
        if let Some(domain) = val.get("domain").and_then(|d| d.as_str()) {
            *counts.entry(domain.to_string()).or_insert(0) += 1;
        }
    }
    counts
}

// --- GET /api/sonar/hotspot ---

#[derive(Serialize)]
pub struct Hotspot {
    pub domain: String,
    pub count: usize,
}

pub async fn get_hotspot() -> Json<Vec<Hotspot>> {
    let counts = count_domains();
    let mut list: Vec<Hotspot> = counts
        .into_iter()
        .map(|(domain, count)| Hotspot { domain, count })
        .collect();
    list.sort_by(|a, b| b.count.cmp(&a.count));
    Json(list)
}

// --- GET /api/sonar/scan ---

#[derive(Serialize)]
pub struct DomainDensity {
    pub domain: String,
    pub count: usize,
    pub grade: String,
}

fn density_grade(count: usize) -> &'static str {
    if count >= 100 {
        "strong"
    } else if count >= 20 {
        "medium"
    } else if count >= 1 {
        "weak"
    } else {
        "void"
    }
}

pub async fn get_scan() -> Json<Vec<DomainDensity>> {
    let counts = count_domains();
    let mut list: Vec<DomainDensity> = counts
        .into_iter()
        .map(|(domain, count)| DomainDensity {
            domain,
            count,
            grade: density_grade(count).to_string(),
        })
        .collect();
    list.sort_by(|a, b| b.count.cmp(&a.count));
    Json(list)
}
