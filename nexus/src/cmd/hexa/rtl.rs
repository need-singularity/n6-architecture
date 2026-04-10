// hexa-rtl — RTL 설계/시뮬 (Makefile 기반)
// 원본: tools/hexa-rtl/
// 사용: nexus hexa rtl [target]  →  make -C tools/hexa-rtl <target>

use std::process::Command;

pub fn run(args: &[String]) -> Result<(), String> {
    let dir = super::tools_dir("hexa-rtl");
    let status = Command::new("make")
        .arg("-C")
        .arg(&dir)
        .args(args)
        .status()
        .map_err(|e| format!("make 실행 실패: {}", e))?;
    if !status.success() {
        return Err(format!("make 비정상 종료: {:?}", status.code()));
    }
    Ok(())
}
