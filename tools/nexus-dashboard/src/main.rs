use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
mod data;
mod routes;
mod targets;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::dashboard::serve_dashboard))
        .route("/api/dimensions", get(routes::dimensions::get_dimensions))
        .route("/api/history", get(routes::history::get_history))
        .route("/api/recent", get(routes::recent::get_recent))
        .route("/api/benchmark", get(routes::benchmark::get_benchmark))
        .route("/api/logs/stream", get(routes::logs::stream_logs))
        .route("/api/daemon/status", get(routes::daemon::get_status))
        .route("/api/daemon/start", post(routes::daemon::start_daemon))
        .route("/api/daemon/stop", post(routes::daemon::stop_daemon))
        .route("/api/grow/{dimension}", post(routes::daemon::grow_dimension))
        .route("/api/sonar/hotspot", get(routes::sonar::get_hotspot))
        .route("/api/sonar/scan", get(routes::sonar::get_scan));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6600));
    println!("NEXUS-6 Dashboard -> http://localhost:6600");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
