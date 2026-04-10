// hexa-ssh — 원격 HEXA 실행 브리지 (Python)
// 원본: tools/hexa-ssh/hexa_ssh.py

use std::process::Command;

pub fn run(args: &[String]) -> Result<(), String> {
    let dir = super::tools_dir("hexa-ssh");
    let script = format!("{}/hexa_ssh.py", dir);
    let status = Command::new("python3")
        .arg(&script)
        .args(args)
        .status()
        .map_err(|e| format!("python3 실행 실패: {}", e))?;
    if !status.success() {
        return Err(format!("hexa-ssh 비정상 종료: {:?}", status.code()));
    }
    Ok(())
}
