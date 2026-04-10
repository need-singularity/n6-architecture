// hexa 카테고리 — RTL/SSH/SIM/Ready 등 이기종 서브도구
//
// 이 카테고리는 Rust 가 아닌 Makefile·Python·RTL 자산을 감싼다.
// 각 서브모듈은 적절한 서브프로세스를 호출한다.

pub mod rtl;
pub mod ssh;
pub mod sim;
pub mod ready_absorber;

pub const NAMES: &[&str] = &["rtl", "ssh", "sim", "ready-absorber"];

pub fn run(name: &str, args: &[String]) -> Result<(), String> {
    match name {
        "rtl" | "hexa-rtl" => rtl::run(args),
        "ssh" | "hexa-ssh" => ssh::run(args),
        "sim" | "hexa-sim" => sim::run(args),
        "ready-absorber" | "ready_absorber" | "absorber" => ready_absorber::run(args),
        _ => Err(format!("hexa 하위 이름을 찾지 못함: {}", name)),
    }
}

/// tools/<name> 절대 경로 해석
pub(crate) fn tools_dir(name: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/ghost".into());
    let candidates = [
        format!("tools/{}", name),
        format!("../tools/{}", name),
        format!("{}/Dev/n6-architecture/tools/{}", home, name),
    ];
    for c in &candidates {
        if std::path::Path::new(c).exists() {
            return c.clone();
        }
    }
    candidates.last().unwrap().clone()
}
