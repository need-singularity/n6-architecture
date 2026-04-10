// ready-absorber — ~/Dev/ready/ 백업 흡수 및 성장 (Python)
// 원본: tools/ready-absorber/absorber.py, verify_and_grow.py

use std::process::Command;

pub fn run(args: &[String]) -> Result<(), String> {
    let dir = super::tools_dir("ready-absorber");
    // 기본은 absorber.py, 첫 인자가 "verify" 면 verify_and_grow.py
    let script = match args.first().map(|s| s.as_str()) {
        Some("verify") => format!("{}/verify_and_grow.py", dir),
        _ => format!("{}/absorber.py", dir),
    };
    let rest: &[String] = if args.first().map(|s| s.as_str()) == Some("verify") {
        &args[1..]
    } else {
        &args[..]
    };
    let status = Command::new("python3")
        .arg(&script)
        .args(rest)
        .status()
        .map_err(|e| format!("python3 실행 실패: {}", e))?;
    if !status.success() {
        return Err(format!("ready-absorber 비정상 종료: {:?}", status.code()));
    }
    Ok(())
}
