use axum::Json;
use serde::Serialize;
use crate::targets::all_targets;
use crate::data;

#[derive(Serialize)]
pub struct DimensionState {
    pub key: String,
    pub label: String,
    pub current: f64,
    pub target: f64,
    pub pct: f64,
    pub color: String,
}

pub async fn get_dimensions() -> Json<Vec<DimensionState>> {
    let targets = all_targets();
    let metrics = collect_current_metrics();
    let states: Vec<DimensionState> = targets
        .iter()
        .map(|t| {
            let current = metrics.get(&t.key).copied().unwrap_or(0.0);
            let pct = if t.target > 0.0 {
                (current / t.target * 100.0).min(100.0)
            } else {
                0.0
            };
            DimensionState {
                key: t.key.clone(),
                label: t.label.clone(),
                current,
                target: t.target,
                pct,
                color: t.color.clone(),
            }
        })
        .collect();
    Json(states)
}

fn collect_current_metrics() -> std::collections::HashMap<String, f64> {
    let mut m = std::collections::HashMap::new();

    // Primary: try structured JSON (strategy.json has dimension scores)
    if let Ok(val) = data::read_json_file("strategy.json") {
        if let Some(dims) = val.get("dimensions").and_then(|d| d.as_object()) {
            for (key, v) in dims {
                if let Some(n) = v.as_f64() {
                    m.insert(key.clone(), n);
                }
            }
        }
    }

    // If JSON didn't provide data, fallback to log parsing
    if m.is_empty() {
        let lines = data::read_daemon_log_lines(100);
        for line in &lines {
            if line.contains('|')
                && !line.contains("Dimension")
                && !line.contains("Overall")
                && !line.contains("+--")
                && !line.contains("+-")
            {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 4 {
                    let key = parts[1].trim().to_lowercase().replace(' ', "_");
                    if let Ok(val) = parts[2].trim().parse::<f64>() {
                        m.insert(key, val);
                    }
                }
            }
        }
    }

    m
}
