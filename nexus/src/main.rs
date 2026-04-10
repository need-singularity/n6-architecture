// NEXUS-6 통합 바이너리 진입점
//
// 두 가지 경로:
//   1) 기존 NEXUS-6 CLI (scan/verify/evolve 등) — cli::parser 가 처리
//   2) tools/ 흡수 dispatcher — `nexus <category> <name> [args...]`
//      category ∈ { calc, dse, analyze, hexa, dashboard, catalog }

use std::env;
use std::process;

const CATEGORIES: &[&str] = &["calc", "dse", "analyze", "hexa", "dashboard", "catalog"];

fn main() {
    let args: Vec<String> = env::args().collect();

    // 1) tools 통합 dispatcher 우선 판별
    if let Some(first) = args.get(1) {
        if CATEGORIES.contains(&first.as_str()) {
            let category = first.as_str();
            if category == "catalog" {
                nexus::cmd::print_catalog();
                return;
            }
            if category == "dashboard" {
                if let Err(e) = nexus::cmd::dashboard::run(&args[2..]) {
                    eprintln!("오류: {}", e);
                    process::exit(1);
                }
                return;
            }
            let name = match args.get(2) {
                Some(n) => n.as_str(),
                None => {
                    eprintln!("오류: `nexus {} <name>` 형식입니다", category);
                    nexus::cmd::print_catalog();
                    process::exit(1);
                }
            };
            let rest: Vec<String> = args.iter().skip(3).cloned().collect();
            if let Err(e) = nexus::cmd::dispatch(category, name, &rest) {
                eprintln!("오류: {}", e);
                process::exit(1);
            }
            return;
        }
    }

    // 2) 기존 NEXUS-6 CLI 경로
    match nexus::cli::parser::parse_args(&args) {
        Ok(cmd) => {
            if let Err(e) = nexus::cli::runner::run(cmd) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!();
            nexus::cli::runner::run(nexus::cli::parser::CliCommand::Help).ok();
            process::exit(1);
        }
    }
}
