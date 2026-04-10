// NEXUS-6 서브커맨드 dispatcher
// 카테고리별 계산기/분석기/DSE/HEXA 도구를 단일 바이너리로 노출
//
// 사용 예:
//   nexus calc optics --lens 6
//   nexus dse universal --seed 1
//   nexus analyze deep-miner
//   nexus hexa rtl --top gate
//
// 원본 tools/<name>/main.hexa 는 HEXA-FIRST 규칙에 따라 .hexa 파일이며
// 여기서는 hexa-lang 바이너리를 subprocess 로 호출하는 얇은 래퍼를 제공한다.

pub mod calc;
pub mod dse;
pub mod analyze;
pub mod hexa;
pub mod dashboard;

use std::process::Command;

/// hexa-lang 바이너리 경로 (세션 환경변수 HEXA_BIN 우선, 없으면 기본 경로)
pub fn hexa_bin_path() -> String {
    std::env::var("HEXA_BIN").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/ghost".into());
        format!("{}/Dev/hexa-lang/target/release/hexa", home)
    })
}

/// tools/<name>/main.hexa 경로 해석
pub fn hexa_source_path(name: &str) -> String {
    // nexus 바이너리가 repo root 혹은 nexus/ 안에서 실행될 수 있으므로
    // 두 경로를 모두 시도한다.
    let candidates = [
        format!("tools/{}/main.hexa", name),
        format!("../tools/{}/main.hexa", name),
        format!("{}/Dev/n6-architecture/tools/{}/main.hexa",
                std::env::var("HOME").unwrap_or_else(|_| "/Users/ghost".into()),
                name),
    ];
    for c in &candidates {
        if std::path::Path::new(c).exists() {
            return c.clone();
        }
    }
    // 존재하지 않으면 마지막 후보를 그대로 반환 (에러는 호출부에서)
    candidates.last().unwrap().clone()
}

/// HEXA 계산기 호출 공통 래퍼
/// - 원본 main.hexa 를 hexa-lang 바이너리로 실행
/// - 추가 args 는 stdin/환경변수로 전달 (현재는 hexa가 stdin 미지원이므로 후속 과제)
pub fn run_hexa_calc(name: &str, args: &[String]) -> Result<(), String> {
    let bin = hexa_bin_path();
    let src = hexa_source_path(name);

    if !std::path::Path::new(&bin).exists() {
        return Err(format!(
            "hexa-lang 바이너리 없음: {} (HEXA_BIN 환경변수로 경로 지정 가능)",
            bin
        ));
    }
    if !std::path::Path::new(&src).exists() {
        return Err(format!("HEXA 소스 없음: {}", src));
    }

    let status = Command::new(&bin)
        .arg(&src)
        .args(args)
        .status()
        .map_err(|e| format!("hexa 실행 실패: {}", e))?;

    if !status.success() {
        return Err(format!("hexa 종료코드 비정상: {:?}", status.code()));
    }
    Ok(())
}

/// 카테고리/이름 dispatcher — main.rs 에서 호출
pub fn dispatch(category: &str, name: &str, args: &[String]) -> Result<(), String> {
    match category {
        "calc" => calc::run(name, args),
        "dse" => dse::run(name, args),
        "analyze" => analyze::run(name, args),
        "hexa" => hexa::run(name, args),
        "dashboard" => dashboard::run(args),
        _ => Err(format!("알 수 없는 카테고리: {}", category)),
    }
}

/// 전체 도구 목록 출력
pub fn print_catalog() {
    println!("NEXUS-6 통합 도구 카탈로그");
    println!("==========================");
    println!();
    println!("[calc]     {}", calc::NAMES.join(", "));
    println!();
    println!("[dse]      {}", dse::NAMES.join(", "));
    println!();
    println!("[analyze]  {}", analyze::NAMES.join(", "));
    println!();
    println!("[hexa]     {}", hexa::NAMES.join(", "));
    println!();
    println!("[기타]     dashboard");
    println!();
    println!("사용법: nexus <category> <name> [args...]");
    println!("        nexus catalog             — 본 목록");
}
