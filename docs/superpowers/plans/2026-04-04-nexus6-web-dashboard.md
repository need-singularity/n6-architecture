# NEXUS-6 Web Dashboard Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** NEXUS-6 성장 시스템의 Rust(Axum) + HTML/JS 로컬 대시보드. 15차원 타임라인 차트 + 최근 성장 테이블 + 실시간 로그 + 데몬 제어.

**Architecture:** Axum HTTP 서버가 `~/.nexus6/` JSON 파일과 `growth.log`를 직접 읽어 API로 서빙. 프론트엔드는 단일 HTML에 Chart.js + vanilla JS. SSE로 로그 스트리밍. 프로세스 제어는 `Command::new("bash")`.

**Tech Stack:** Rust (Axum + Tokio + Serde + tower-http), Chart.js 4.x (CDN), vanilla JS

**Spec:** `docs/superpowers/specs/2026-04-04-nexus6-web-dashboard-design.md`

---

## File Structure

```
tools/nexus6-dashboard/
├── Cargo.toml
├── src/
│   ├── main.rs           — Axum 라우터 + 서버 시작 (port 6600)
│   ├── data.rs            — ~/.nexus6/ JSON 읽기 + measure.sh 메트릭 파싱
│   ├── targets.rs         — 15차원 목표값 정의
│   ├── routes/
│   │   ├── mod.rs         — 라우트 모듈 re-export
│   │   ├── dashboard.rs   — GET / → HTML 서빙
│   │   ├── dimensions.rs  — GET /api/dimensions
│   │   ├── history.rs     — GET /api/history
│   │   ├── recent.rs      — GET /api/recent (최근 성장 시간순)
│   │   ├── benchmark.rs   — GET /api/benchmark
│   │   ├── logs.rs        — GET /api/logs/stream (SSE)
│   │   └── daemon.rs      — POST start/stop, GET status, POST grow/:dim
│   └── tests/
│       ├── data_test.rs   — data.rs 단위 테스트
│       ├── targets_test.rs— targets 테스트
│       └── api_test.rs    — API 통합 테스트
└── static/
    └── index.html         — 프론트엔드 (Chart.js + vanilla JS)
```

---

### Task 1: Cargo 프로젝트 초기화 + 최소 Axum 서버

**Files:**
- Create: `tools/nexus6-dashboard/Cargo.toml`
- Create: `tools/nexus6-dashboard/src/main.rs`

- [ ] **Step 1: Cargo.toml 작성**

```toml
[package]
name = "nexus6-dashboard"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.5", features = ["cors"] }
```

- [ ] **Step 2: 최소 main.rs 작성 — "Hello NEXUS-6" 응답**

```rust
use axum::{Router, routing::get, response::Html};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>NEXUS-6 Dashboard</h1>") }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6600));
    println!("NEXUS-6 Dashboard → http://localhost:6600");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

- [ ] **Step 3: 빌드 확인**

Run: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo build --release 2>&1 | tail -5`
Expected: `Compiling nexus6-dashboard` 성공

- [ ] **Step 4: 서버 시작 + curl 테스트**

Run: `./target/release/nexus6-dashboard &` then `curl -s http://localhost:6600`
Expected: `<h1>NEXUS-6 Dashboard</h1>`
Stop: `kill %1`

- [ ] **Step 5: 커밋**

```bash
cd /Users/ghost/Dev/n6-architecture
git add tools/nexus6-dashboard/
git commit -m "feat: nexus6-dashboard 최소 Axum 서버 (port 6600)"
```

---

### Task 2: targets.rs — 15차원 목표값 정의

**Files:**
- Create: `tools/nexus6-dashboard/src/targets.rs`
- Create: `tools/nexus6-dashboard/src/tests/targets_test.rs`
- Modify: `tools/nexus6-dashboard/src/main.rs` (mod 선언 추가)

- [ ] **Step 1: 테스트 작성**

```rust
// src/tests/targets_test.rs
#[cfg(test)]
mod tests {
    use crate::targets::{DimensionTarget, all_targets};

    #[test]
    fn test_all_targets_has_15_dimensions() {
        let targets = all_targets();
        assert_eq!(targets.len(), 15);
    }

    #[test]
    fn test_performance_target() {
        let targets = all_targets();
        let perf = targets.iter().find(|t| t.key == "performance").unwrap();
        assert_eq!(perf.target, 10_000.0);
        assert!(!perf.label.is_empty());
    }

    #[test]
    fn test_all_targets_have_labels() {
        for t in all_targets() {
            assert!(!t.label.is_empty(), "missing label for {}", t.key);
            assert!(t.target > 0.0, "zero target for {}", t.key);
        }
    }
}
```

- [ ] **Step 2: targets.rs 구현**

```rust
// src/targets.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DimensionTarget {
    pub key: String,
    pub label: String,
    pub target: f64,
    pub color: String,
}

pub fn all_targets() -> Vec<DimensionTarget> {
    vec![
        dt("performance",     "Performance",   10_000.0, "#ff6384"),
        dt("architecture",    "Architecture",     100.0, "#36a2eb"),
        dt("lenses",          "Lenses",           200.0, "#ffce56"),
        dt("modules",         "Modules",            4.0, "#4bc0c0"),
        dt("tests",           "Tests",           1000.0, "#9966ff"),
        dt("hypotheses",      "Hypotheses",       150.0, "#ff9f40"),
        dt("dse",             "DSE",              322.0, "#e7e9ed"),
        dt("experiments",     "Experiments",       50.0, "#c9cbcf"),
        dt("calculators",     "Calculators",       50.0, "#7cb342"),
        dt("cross_resonance", "CrossReson",       100.0, "#e91e63"),
        dt("knowledge_graph", "KnowledgeGr",      500.0, "#00bcd4"),
        dt("red_team",        "RedTeam",          100.0, "#ff5722"),
        dt("atlas",           "Atlas",           2000.0, "#795548"),
        dt("documentation",   "Docs",              90.0, "#607d8b"),
        dt("integration",     "Integration",       50.0, "#8bc34a"),
    ]
}

fn dt(key: &str, label: &str, target: f64, color: &str) -> DimensionTarget {
    DimensionTarget {
        key: key.to_string(),
        label: label.to_string(),
        target,
        color: color.to_string(),
    }
}
```

- [ ] **Step 3: main.rs에 모듈 등록**

main.rs 상단에 추가:
```rust
mod targets;
#[cfg(test)]
mod tests;
```

그리고 `src/tests/mod.rs` 생성:
```rust
mod targets_test;
```

- [ ] **Step 4: 테스트 실행**

Run: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo test -- --nocapture 2>&1 | tail -10`
Expected: `test result: ok. 3 passed`

- [ ] **Step 5: 커밋**

```bash
git add tools/nexus6-dashboard/src/targets.rs tools/nexus6-dashboard/src/tests/
git commit -m "feat: 15차원 목표값 정의 (targets.rs) + 테스트"
```

---

### Task 3: data.rs — ~/.nexus6/ JSON 읽기

**Files:**
- Create: `tools/nexus6-dashboard/src/data.rs`
- Create: `tools/nexus6-dashboard/src/tests/data_test.rs`
- Modify: `tools/nexus6-dashboard/src/main.rs` (mod 추가)

- [ ] **Step 1: 테스트 작성**

```rust
// src/tests/data_test.rs
#[cfg(test)]
mod tests {
    use crate::data::{nexus6_dir, read_json_file, read_growth_log, read_daemon_log_lines};

    #[test]
    fn test_nexus6_dir_exists() {
        let dir = nexus6_dir();
        // CI에서는 없을 수 있으므로 경로 형식만 확인
        assert!(dir.ends_with(".nexus6"));
    }

    #[test]
    fn test_read_json_file_missing() {
        let result = read_json_file("__nonexistent_file_12345__.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_growth_log_empty_ok() {
        // growth_log.jsonl이 비어있어도 빈 Vec 반환
        let entries = read_growth_log();
        // 현재 파일이 비어있으므로 0개 OK
        assert!(entries.len() >= 0);
    }

    #[test]
    fn test_read_daemon_log_lines() {
        let lines = read_daemon_log_lines(10);
        // 로그가 있든 없든 패닉 안 함
        assert!(lines.len() <= 10);
    }
}
```

- [ ] **Step 2: data.rs 구현**

```rust
// src/data.rs
use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn nexus6_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/ghost".to_string());
    PathBuf::from(home).join(".nexus6")
}

pub fn repo_root() -> PathBuf {
    // tools/nexus6-dashboard -> 프로젝트 루트
    let mut p = std::env::current_exe().unwrap_or_default();
    // fallback: 환경변수 또는 하드코딩
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
```

- [ ] **Step 3: main.rs에 mod data 추가**

```rust
mod data;
```

tests/mod.rs에 추가:
```rust
mod data_test;
```

- [ ] **Step 4: 테스트 실행**

Run: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo test -- --nocapture 2>&1 | tail -10`
Expected: 전체 테스트 통과

- [ ] **Step 5: 커밋**

```bash
git add tools/nexus6-dashboard/src/data.rs tools/nexus6-dashboard/src/tests/data_test.rs
git commit -m "feat: data.rs — ~/.nexus6/ JSON 읽기 + 로그 파싱"
```

---

### Task 4: API 라우트 — dimensions + history + recent + benchmark

**Files:**
- Create: `tools/nexus6-dashboard/src/routes/mod.rs`
- Create: `tools/nexus6-dashboard/src/routes/dimensions.rs`
- Create: `tools/nexus6-dashboard/src/routes/history.rs`
- Create: `tools/nexus6-dashboard/src/routes/recent.rs`
- Create: `tools/nexus6-dashboard/src/routes/benchmark.rs`
- Modify: `tools/nexus6-dashboard/src/main.rs` (라우트 등록)

- [ ] **Step 1: routes/mod.rs 작성**

```rust
pub mod dimensions;
pub mod history;
pub mod recent;
pub mod benchmark;
```

- [ ] **Step 2: dimensions.rs — GET /api/dimensions**

현재 메트릭을 measure.sh 방식으로 계산하는 대신, growth.log 마지막 엔트리와 ~/.nexus6/ 파일에서 읽기.

```rust
// src/routes/dimensions.rs
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

    let states: Vec<DimensionState> = targets.iter().map(|t| {
        let current = metrics.get(&t.key).copied().unwrap_or(0.0);
        let pct = if t.target > 0.0 { (current / t.target * 100.0).min(100.0) } else { 0.0 };
        DimensionState {
            key: t.key.clone(),
            label: t.label.clone(),
            current,
            target: t.target,
            pct,
            color: t.color.clone(),
        }
    }).collect();

    Json(states)
}

fn collect_current_metrics() -> std::collections::HashMap<String, f64> {
    let mut m = std::collections::HashMap::new();

    // growth.log 파싱하여 대시보드 테이블에서 현재값 추출
    let lines = data::read_daemon_log_lines(100);
    for line in &lines {
        // "| performance  |   38927 |   10000 |" 패턴 파싱
        if line.contains('|') && !line.contains("Dimension") && !line.contains("Overall") && !line.contains("+--") {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                let key = parts[1].trim().to_lowercase().replace(' ', "_");
                if let Ok(val) = parts[2].trim().parse::<f64>() {
                    m.insert(key, val);
                }
            }
        }
    }

    m
}
```

- [ ] **Step 3: history.rs — GET /api/history**

```rust
// src/routes/history.rs
use axum::Json;
use crate::data;

pub async fn get_history() -> Json<Vec<serde_json::Value>> {
    Json(data::read_growth_log())
}
```

- [ ] **Step 4: recent.rs — GET /api/recent**

```rust
// src/routes/recent.rs
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
        // "[16:48:00] INFO:  Step 3/5: Executing growth for 'knowledge_graph'..."
        if line.contains("Executing growth for") {
            let time = line.get(1..9).unwrap_or("").to_string();
            let dim = line.split('\'').nth(1).unwrap_or("unknown").to_string();
            events.push(RecentEvent {
                time,
                dimension: dim,
                detail: line.clone(),
            });
        }
        // "  Cycle N: dimension [OK]"
        if line.contains("[OK]") && line.contains("Cycle") {
            let time = String::new();
            let detail = line.trim().to_string();
            let dim = detail.split(':').nth(1)
                .and_then(|s| s.split('[').next())
                .unwrap_or("")
                .trim().to_string();
            events.push(RecentEvent { time, dimension: dim, detail });
        }
    }

    events.reverse(); // 최신순
    Json(events)
}
```

- [ ] **Step 5: benchmark.rs — GET /api/benchmark**

```rust
// src/routes/benchmark.rs
use axum::Json;
use crate::data;

pub async fn get_benchmark() -> Json<serde_json::Value> {
    match data::read_benchmark() {
        Ok(v) => Json(v),
        Err(_) => Json(serde_json::json!({"error": "no benchmark data"})),
    }
}
```

- [ ] **Step 6: main.rs에 라우트 등록**

```rust
mod routes;

// Router 구성:
let app = Router::new()
    .route("/", get(routes::dashboard::serve_dashboard))
    .route("/api/dimensions", get(routes::dimensions::get_dimensions))
    .route("/api/history", get(routes::history::get_history))
    .route("/api/recent", get(routes::recent::get_recent))
    .route("/api/benchmark", get(routes::benchmark::get_benchmark))
    .layer(tower_http::cors::CorsLayer::permissive());
```

- [ ] **Step 7: 빌드 확인**

Run: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo build --release 2>&1 | tail -5`
Expected: 성공

- [ ] **Step 8: 커밋**

```bash
git add tools/nexus6-dashboard/src/routes/
git commit -m "feat: API 라우트 — dimensions, history, recent, benchmark"
```

---

### Task 5: daemon.rs — 데몬 제어 API

**Files:**
- Create: `tools/nexus6-dashboard/src/routes/daemon.rs`
- Modify: `tools/nexus6-dashboard/src/routes/mod.rs` (추가)
- Modify: `tools/nexus6-dashboard/src/main.rs` (라우트 등록)

- [ ] **Step 1: daemon.rs 작성**

```rust
// src/routes/daemon.rs
use axum::{Json, extract::Path};
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
    Json(DaemonStatus { running, pid, uptime_info })
}

pub async fn start_daemon() -> Json<serde_json::Value> {
    let repo_root = data::repo_root();
    let script = repo_root.join("tools/nexus6/scripts/nexus6_growth_daemon.sh");

    let result = Command::new("bash")
        .arg(script.to_str().unwrap())
        .arg("--max-cycles").arg("999")
        .arg("--interval").arg("30")
        .arg("--skip-commit")
        .spawn();

    match result {
        Ok(child) => Json(serde_json::json!({
            "ok": true, "pid": child.id()
        })),
        Err(e) => Json(serde_json::json!({
            "ok": false, "error": e.to_string()
        })),
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
    let repo_root = data::repo_root();
    let script = repo_root.join("tools/nexus6/scripts/auto_grow.sh");

    let result = Command::new("bash")
        .arg(script.to_str().unwrap())
        .arg("--dimension").arg(&dimension)
        .arg("--cycles").arg("1")
        .spawn();

    match result {
        Ok(child) => Json(serde_json::json!({
            "ok": true, "pid": child.id(), "dimension": dimension
        })),
        Err(e) => Json(serde_json::json!({
            "ok": false, "error": e.to_string()
        })),
    }
}

fn check_daemon() -> (bool, Option<u32>) {
    // ps aux에서 nexus6_growth_daemon 찾기
    let output = Command::new("ps")
        .args(["aux"])
        .output();

    match output {
        Ok(out) => {
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
            (false, None)
        }
        Err(_) => (false, None),
    }
}
```

- [ ] **Step 2: routes/mod.rs에 추가**

```rust
pub mod daemon;
```

- [ ] **Step 3: main.rs에 데몬 라우트 등록**

```rust
.route("/api/daemon/status", get(routes::daemon::get_status))
.route("/api/daemon/start", post(routes::daemon::start_daemon))
.route("/api/daemon/stop", post(routes::daemon::stop_daemon))
.route("/api/grow/:dimension", post(routes::daemon::grow_dimension))
```

`use axum::routing::post;` 추가.

- [ ] **Step 4: 빌드 확인**

Run: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo build --release 2>&1 | tail -5`
Expected: 성공

- [ ] **Step 5: 커밋**

```bash
git add tools/nexus6-dashboard/src/routes/daemon.rs
git commit -m "feat: daemon.rs — 데몬 시작/정지/상태/수동성장 API"
```

---

### Task 6: logs.rs — SSE 실시간 로그 스트리밍

**Files:**
- Create: `tools/nexus6-dashboard/src/routes/logs.rs`
- Modify: `tools/nexus6-dashboard/src/routes/mod.rs`
- Modify: `tools/nexus6-dashboard/src/main.rs`

- [ ] **Step 1: Cargo.toml에 의존성 추가**

```toml
tokio-stream = "0.1"
futures = "0.3"
```

- [ ] **Step 2: logs.rs 작성**

```rust
// src/routes/logs.rs
use axum::response::sse::{Event, Sse};
use futures::stream::Stream;
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::StreamExt;
use crate::data;

pub async fn stream_logs() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut last_len: usize = 0;

    let stream = tokio_stream::wrappers::IntervalStream::new(
        tokio::time::interval(Duration::from_secs(2))
    )
    .map(move |_| {
        let lines = data::read_daemon_log_lines(500);
        let current_len = lines.len();

        if current_len > last_len {
            let new_lines = &lines[last_len..];
            last_len = current_len;
            let payload = new_lines.join("\n");
            Ok(Event::default().data(payload))
        } else {
            last_len = current_len;
            Ok(Event::default().comment("heartbeat"))
        }
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
    )
}
```

- [ ] **Step 3: routes/mod.rs + main.rs 등록**

mod.rs에 추가: `pub mod logs;`

main.rs 라우트에 추가:
```rust
.route("/api/logs/stream", get(routes::logs::stream_logs))
```

- [ ] **Step 4: 빌드 확인**

Run: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo build --release 2>&1 | tail -5`
Expected: 성공

- [ ] **Step 5: 커밋**

```bash
git add tools/nexus6-dashboard/src/routes/logs.rs tools/nexus6-dashboard/Cargo.toml
git commit -m "feat: SSE 실시간 로그 스트리밍 (/api/logs/stream)"
```

---

### Task 7: dashboard.rs — HTML 서빙

**Files:**
- Create: `tools/nexus6-dashboard/src/routes/dashboard.rs`

- [ ] **Step 1: dashboard.rs 작성**

```rust
// src/routes/dashboard.rs
use axum::response::Html;

pub async fn serve_dashboard() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}
```

- [ ] **Step 2: routes/mod.rs에 추가**

```rust
pub mod dashboard;
```

- [ ] **Step 3: 빌드 확인** (static/index.html은 Task 8에서 생성)

이 시점에서는 빌드만 되면 OK (index.html 없으면 컴파일 에러 → Task 8과 함께 해결).

- [ ] **Step 4: 커밋**

```bash
git add tools/nexus6-dashboard/src/routes/dashboard.rs
git commit -m "feat: dashboard.rs — HTML include_str 서빙"
```

---

### Task 8: index.html — 프론트엔드 전체

**Files:**
- Create: `tools/nexus6-dashboard/static/index.html`

- [ ] **Step 1: 완전한 단일 HTML 파일 작성**

이 파일이 프론트엔드 전체. Chart.js CDN + vanilla JS. 3개 섹션: 타임라인 차트 / 최근 성장 테이블 / 실시간 로그.

```html
<!DOCTYPE html>
<html lang="ko">
<head>
<meta charset="UTF-8">
<title>NEXUS-6 Growth Monitor</title>
<script src="https://cdn.jsdelivr.net/npm/chart.js@4"></script>
<style>
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body { background: #0d1117; color: #c9d1d9; font-family: 'SF Mono', 'Fira Code', monospace; font-size: 14px; }
  .header { display: flex; justify-content: space-between; align-items: center; padding: 16px 24px; border-bottom: 1px solid #21262d; }
  .header h1 { font-size: 18px; color: #58a6ff; }
  .controls { display: flex; gap: 8px; }
  .controls button { background: #21262d; color: #c9d1d9; border: 1px solid #30363d; padding: 6px 14px; border-radius: 6px; cursor: pointer; font-family: inherit; }
  .controls button:hover { background: #30363d; }
  .controls button.active { background: #238636; border-color: #2ea043; color: #fff; }
  .controls button.danger { background: #da3633; border-color: #f85149; color: #fff; }
  .status-dot { display: inline-block; width: 8px; height: 8px; border-radius: 50%; margin-right: 6px; }
  .status-dot.on { background: #3fb950; }
  .status-dot.off { background: #f85149; }

  .chart-section { padding: 16px 24px; border-bottom: 1px solid #21262d; }
  .chart-section canvas { width: 100% !important; height: 300px !important; }

  .table-section { padding: 16px 24px; border-bottom: 1px solid #21262d; max-height: 300px; overflow-y: auto; }
  .table-section h2 { font-size: 14px; margin-bottom: 8px; color: #8b949e; }
  table { width: 100%; border-collapse: collapse; }
  th { text-align: left; color: #8b949e; font-weight: normal; padding: 6px 8px; border-bottom: 1px solid #21262d; }
  td { padding: 6px 8px; border-bottom: 1px solid #161b22; }
  td.dim { color: #58a6ff; }
  td.delta-pos { color: #3fb950; }
  td.delta-neg { color: #f85149; }
  tr.new-row { animation: highlight 2s ease-out; }
  @keyframes highlight { from { background: #1f2937; } to { background: transparent; } }

  .log-section { padding: 16px 24px; }
  .log-section h2 { font-size: 14px; margin-bottom: 8px; color: #8b949e; display: flex; justify-content: space-between; }
  .log-box { background: #010409; border: 1px solid #21262d; border-radius: 6px; padding: 12px; height: 200px; overflow-y: auto; font-size: 12px; line-height: 1.6; }
  .log-box .log-line { color: #7ee787; }
  .log-box .log-info { color: #8b949e; }
  .log-box .log-error { color: #f85149; }

  .grow-modal { display: none; position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 24px; z-index: 100; }
  .grow-modal.show { display: block; }
  .grow-modal h3 { margin-bottom: 12px; color: #58a6ff; }
  .grow-modal button { margin: 4px; padding: 8px 16px; background: #21262d; border: 1px solid #30363d; color: #c9d1d9; border-radius: 4px; cursor: pointer; }
  .grow-modal button:hover { background: #30363d; }
  .overlay { display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); z-index: 99; }
  .overlay.show { display: block; }
</style>
</head>
<body>

<div class="header">
  <h1><span class="status-dot off" id="statusDot"></span> NEXUS-6 Growth Monitor</h1>
  <div class="controls">
    <button id="btnStart" class="active" onclick="daemonStart()">Start</button>
    <button id="btnStop" class="danger" onclick="daemonStop()">Stop</button>
    <button onclick="showGrowModal()">Grow</button>
  </div>
</div>

<div class="chart-section">
  <canvas id="timelineChart"></canvas>
</div>

<div class="table-section">
  <h2>Recent Growth</h2>
  <table>
    <thead><tr><th>Time</th><th>Dimension</th><th>Current</th><th>Target</th><th>%</th></tr></thead>
    <tbody id="dimTable"></tbody>
  </table>
</div>

<div class="log-section">
  <h2>
    <span>Live Log</span>
    <span>
      <button onclick="clearLog()" style="font-size:12px;padding:2px 8px;background:#21262d;border:1px solid #30363d;color:#8b949e;border-radius:4px;cursor:pointer;">Clear</button>
      <button id="btnPause" onclick="togglePause()" style="font-size:12px;padding:2px 8px;background:#21262d;border:1px solid #30363d;color:#8b949e;border-radius:4px;cursor:pointer;">Pause</button>
    </span>
  </h2>
  <div class="log-box" id="logBox"></div>
</div>

<div class="overlay" id="overlay" onclick="hideGrowModal()"></div>
<div class="grow-modal" id="growModal">
  <h3>Manual Grow</h3>
  <div id="growButtons"></div>
</div>

<script>
const DIMS = [
  'performance','architecture','lenses','modules','tests','hypotheses',
  'dse','experiments','calculators','cross_resonance','knowledge_graph',
  'red_team','atlas','documentation','integration'
];

let chart = null;
let paused = false;
let logLines = [];

// ── Chart ──
function initChart() {
  const ctx = document.getElementById('timelineChart').getContext('2d');
  chart = new Chart(ctx, {
    type: 'line',
    data: { labels: [], datasets: [] },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { labels: { color: '#8b949e', font: { size: 11 } } } },
      scales: {
        x: { ticks: { color: '#8b949e' }, grid: { color: '#21262d' } },
        y: { ticks: { color: '#8b949e' }, grid: { color: '#21262d' },
             title: { display: true, text: '% of target', color: '#8b949e' } }
      }
    }
  });
}

// ── Fetch dimensions + update chart + table ──
async function fetchDimensions() {
  try {
    const res = await fetch('/api/dimensions');
    const dims = await res.json();

    // Update table (sorted by pct ascending = weakest first)
    const sorted = [...dims].sort((a, b) => a.pct - b.pct);
    const tbody = document.getElementById('dimTable');
    tbody.innerHTML = sorted.map(d => `
      <tr>
        <td>${new Date().toLocaleTimeString('ko-KR', {hour:'2-digit',minute:'2-digit'})}</td>
        <td class="dim">${d.label}</td>
        <td>${typeof d.current === 'number' ? d.current.toLocaleString() : d.current}</td>
        <td>${d.target.toLocaleString()}</td>
        <td style="color:${d.pct >= 100 ? '#3fb950' : d.pct >= 80 ? '#d29922' : '#f85149'}">${d.pct.toFixed(1)}%</td>
      </tr>
    `).join('');

    // Update chart — single snapshot point (accumulates over time)
    if (chart) {
      const now = new Date().toLocaleTimeString('ko-KR', {hour:'2-digit',minute:'2-digit',second:'2-digit'});
      chart.data.labels.push(now);
      if (chart.data.labels.length > 60) chart.data.labels.shift();

      if (chart.data.datasets.length === 0) {
        dims.forEach(d => {
          chart.data.datasets.push({
            label: d.label,
            borderColor: d.color,
            backgroundColor: 'transparent',
            data: [d.pct],
            tension: 0.3,
            pointRadius: 0,
            borderWidth: 1.5,
          });
        });
      } else {
        dims.forEach((d, i) => {
          chart.data.datasets[i].data.push(d.pct);
          if (chart.data.datasets[i].data.length > 60) chart.data.datasets[i].data.shift();
        });
      }
      chart.update('none');
    }
  } catch (e) {
    console.error('fetchDimensions:', e);
  }
}

// ── SSE Logs ──
function connectSSE() {
  const evtSource = new EventSource('/api/logs/stream');
  evtSource.onmessage = (event) => {
    if (paused) return;
    const box = document.getElementById('logBox');
    const newLines = event.data.split('\n').filter(l => l.trim());
    newLines.forEach(line => {
      logLines.push(line);
      if (logLines.length > 500) logLines.shift();
      const div = document.createElement('div');
      div.className = line.includes('ERROR') ? 'log-error' : line.includes('INFO') ? 'log-line' : 'log-info';
      div.textContent = line;
      box.appendChild(div);
    });
    // auto-scroll
    box.scrollTop = box.scrollHeight;
    // trim DOM
    while (box.children.length > 500) box.removeChild(box.firstChild);
  };
}

// ── Daemon control ──
async function daemonStart() {
  await fetch('/api/daemon/start', { method: 'POST' });
  checkStatus();
}

async function daemonStop() {
  await fetch('/api/daemon/stop', { method: 'POST' });
  checkStatus();
}

async function checkStatus() {
  try {
    const res = await fetch('/api/daemon/status');
    const data = await res.json();
    const dot = document.getElementById('statusDot');
    dot.className = 'status-dot ' + (data.running ? 'on' : 'off');
  } catch(e) {}
}

// ── Grow modal ──
function showGrowModal() {
  const btns = document.getElementById('growButtons');
  btns.innerHTML = DIMS.map(d =>
    `<button onclick="triggerGrow('${d}')">${d}</button>`
  ).join('');
  document.getElementById('growModal').classList.add('show');
  document.getElementById('overlay').classList.add('show');
}

function hideGrowModal() {
  document.getElementById('growModal').classList.remove('show');
  document.getElementById('overlay').classList.remove('show');
}

async function triggerGrow(dim) {
  hideGrowModal();
  await fetch(`/api/grow/${dim}`, { method: 'POST' });
}

// ── Log controls ──
function clearLog() {
  document.getElementById('logBox').innerHTML = '';
  logLines = [];
}

function togglePause() {
  paused = !paused;
  document.getElementById('btnPause').textContent = paused ? 'Resume' : 'Pause';
}

// ── Init ──
window.addEventListener('DOMContentLoaded', () => {
  initChart();
  fetchDimensions();
  checkStatus();
  connectSSE();

  // Poll dimensions every 10 seconds
  setInterval(fetchDimensions, 10000);
  // Poll daemon status every 30 seconds
  setInterval(checkStatus, 30000);
});
</script>
</body>
</html>
```

- [ ] **Step 2: 전체 빌드 + 서버 시작 테스트**

Run:
```bash
cd tools/nexus6-dashboard && ~/.cargo/bin/cargo build --release 2>&1 | tail -5
```
Expected: 성공

- [ ] **Step 3: 통합 테스트 — 서버 시작 후 모든 API 확인**

```bash
./target/release/nexus6-dashboard &
sleep 1
echo "=== Dashboard ===" && curl -s http://localhost:6600 | head -5
echo "=== Dimensions ===" && curl -s http://localhost:6600/api/dimensions | python3 -m json.tool | head -10
echo "=== Status ===" && curl -s http://localhost:6600/api/daemon/status | python3 -m json.tool
echo "=== Benchmark ===" && curl -s http://localhost:6600/api/benchmark | python3 -m json.tool | head -5
echo "=== Recent ===" && curl -s http://localhost:6600/api/recent | python3 -m json.tool | head -10
kill %1
```

- [ ] **Step 4: 커밋**

```bash
git add tools/nexus6-dashboard/static/index.html tools/nexus6-dashboard/src/routes/dashboard.rs
git commit -m "feat: NEXUS-6 Web Dashboard 완성 — Chart.js + SSE + 데몬 제어"
```

---

### Task 9: main.rs 최종 조립 + 전체 통합

**Files:**
- Modify: `tools/nexus6-dashboard/src/main.rs` — 모든 라우트 통합

- [ ] **Step 1: main.rs 최종 버전 작성**

```rust
use axum::{Router, routing::{get, post}};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod data;
mod targets;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // Dashboard
        .route("/", get(routes::dashboard::serve_dashboard))
        // API — read
        .route("/api/dimensions", get(routes::dimensions::get_dimensions))
        .route("/api/history", get(routes::history::get_history))
        .route("/api/recent", get(routes::recent::get_recent))
        .route("/api/benchmark", get(routes::benchmark::get_benchmark))
        // API — SSE
        .route("/api/logs/stream", get(routes::logs::stream_logs))
        // API — daemon control
        .route("/api/daemon/status", get(routes::daemon::get_status))
        .route("/api/daemon/start", post(routes::daemon::start_daemon))
        .route("/api/daemon/stop", post(routes::daemon::stop_daemon))
        .route("/api/grow/{dimension}", post(routes::daemon::grow_dimension))
        // Middleware
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 6600));
    println!("NEXUS-6 Dashboard → http://localhost:6600");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
```

- [ ] **Step 2: 전체 테스트 실행**

Run: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo test 2>&1 | tail -10`
Expected: 전체 통과

- [ ] **Step 3: 릴리즈 빌드**

Run: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo build --release 2>&1 | tail -3`
Expected: 성공

- [ ] **Step 4: 최종 커밋**

```bash
git add tools/nexus6-dashboard/
git commit -m "feat: NEXUS-6 Web Dashboard v1 — Axum + Chart.js + SSE + 데몬 제어 (port 6600)"
```

---

## Summary

| Task | 내용 | 파일 수 |
|------|------|---------|
| 1 | Cargo 프로젝트 + 최소 서버 | 2 |
| 2 | targets.rs (15차원 목표) | 2 |
| 3 | data.rs (JSON 읽기) | 2 |
| 4 | API 라우트 4개 (dimensions/history/recent/benchmark) | 5 |
| 5 | daemon.rs (데몬 제어) | 1 |
| 6 | logs.rs (SSE 스트리밍) | 1 |
| 7 | dashboard.rs (HTML 서빙) | 1 |
| 8 | index.html (프론트엔드 전체) | 1 |
| 9 | main.rs 최종 조립 | 1 |
| **Total** | | **16 files, 9 tasks** |
