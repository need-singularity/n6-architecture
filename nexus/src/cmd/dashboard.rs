// dashboard — Axum 기반 NEXUS-6 웹 대시보드 (port 6600)
// 원본: tools/nexus-dashboard  →  nexus/crates/dashboard (워크스페이스 멤버)
//
// 주의: nexus-dashboard 는 별도 크레이트이며 nexus 바이너리와 링크하면
// tokio 런타임이 중복되므로 subprocess 로 호출한다.

use std::process::Command;

pub fn run(args: &[String]) -> Result<(), String> {
    println!("[dashboard] nexus-dashboard 기동 (port 6600)");
    let status = Command::new("cargo")
        .args([
            "run",
            "--release",
            "-p",
            "nexus-dashboard",
            "--",
        ])
        .args(args)
        .status()
        .map_err(|e| format!("cargo 실행 실패: {}", e))?;
    if !status.success() {
        return Err(format!("dashboard 비정상 종료: {:?}", status.code()));
    }
    Ok(())
}
