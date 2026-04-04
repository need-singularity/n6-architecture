use axum::Json;
use serde::Serialize;
use crate::data;

#[derive(Serialize)]
pub struct RecentEvent {
    pub time: String,
    pub dimension: String,
    pub detail: String,
}

pub async fn get_recent() -> Json<Vec<RecentEvent>> {
    let lines = data::read_daemon_log_lines(200);
    let mut events = Vec::new();
    for line in &lines {
        if line.contains("Executing growth for") {
            let time = line.get(1..9).unwrap_or("").to_string();
            let dim = line
                .split('\'')
                .nth(1)
                .unwrap_or("unknown")
                .to_string();
            events.push(RecentEvent {
                time,
                dimension: dim,
                detail: line.clone(),
            });
        }
        if line.contains("[OK]") && line.contains("Cycle") {
            let detail = line.trim().to_string();
            let dim = detail
                .split(':')
                .nth(1)
                .and_then(|s| s.split('[').next())
                .unwrap_or("")
                .trim()
                .to_string();
            events.push(RecentEvent {
                time: String::new(),
                dimension: dim,
                detail,
            });
        }
    }
    events.reverse();
    Json(events)
}
