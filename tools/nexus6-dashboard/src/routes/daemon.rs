use axum::{extract::Path, Json};
use serde::Serialize;
use std::process::Command;
use crate::data;

#[derive(Serialize)]
pub struct DaemonStatus {
    pub running: bool,
    pub pid: Option<u32>,
    pub uptime_info: String,
}

pub async fn get_status() -> Json<DaemonStatus> {
    let (running, pid) = check_daemon();
    let uptime_info = if running {
        format!("PID {} running", pid.unwrap_or(0))
    } else {
        "stopped".to_string()
    };
    Json(DaemonStatus {
        running,
        pid,
        uptime_info,
    })
}

pub async fn start_daemon() -> Json<serde_json::Value> {
    let repo_root = data::repo_root();
    let script = repo_root.join("tools/nexus6/scripts/nexus6_growth_daemon.sh");
    let result = Command::new("bash")
        .arg(script.to_str().unwrap())
        .arg("--max-cycles")
        .arg("999")
        .arg("--interval")
        .arg("30")
        .arg("--skip-commit")
        .spawn();
    match result {
        Ok(child) => Json(serde_json::json!({"ok": true, "pid": child.id()})),
        Err(e) => Json(serde_json::json!({"ok": false, "error": e.to_string()})),
    }
}

pub async fn stop_daemon() -> Json<serde_json::Value> {
    let (running, pid) = check_daemon();
    if !running {
        return Json(serde_json::json!({"ok": true, "msg": "already stopped"}));
    }
    if let Some(p) = pid {
        let _ = Command::new("kill").arg(p.to_string()).status();
    }
    Json(serde_json::json!({"ok": true}))
}

pub async fn grow_dimension(Path(dimension): Path<String>) -> Json<serde_json::Value> {
    let valid = crate::targets::all_targets();
    if !valid.iter().any(|t| t.key == dimension) {
        return Json(serde_json::json!({"ok": false, "error": "unknown dimension"}));
    }
    let repo_root = data::repo_root();
    let script = repo_root.join("tools/nexus6/scripts/auto_grow.sh");
    let result = Command::new("bash")
        .arg(script.to_str().unwrap())
        .arg("--dimension")
        .arg(&dimension)
        .arg("--cycles")
        .arg("1")
        .spawn();
    match result {
        Ok(child) => {
            Json(serde_json::json!({"ok": true, "pid": child.id(), "dimension": dimension}))
        }
        Err(e) => Json(serde_json::json!({"ok": false, "error": e.to_string()})),
    }
}

fn check_daemon() -> (bool, Option<u32>) {
    let pid_path = data::nexus6_dir().join("daemon.pid");
    if let Ok(content) = std::fs::read_to_string(&pid_path) {
        if let Ok(pid) = content.trim().parse::<u32>() {
            let status = Command::new("kill").args(["-0", &pid.to_string()]).status();
            if status.map(|s| s.success()).unwrap_or(false) {
                return (true, Some(pid));
            }
        }
    }
    // Fallback: ps grep
    let output = Command::new("ps").args(["aux"]).output();
    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            if line.contains("nexus6_growth_daemon") && !line.contains("grep") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(pid) = parts[1].parse::<u32>() {
                        return (true, Some(pid));
                    }
                }
            }
        }
    }
    (false, None)
}
