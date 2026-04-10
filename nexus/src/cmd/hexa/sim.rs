// hexa-sim — 분석/시뮬 벤치마크 자료 디렉터리
// 원본: tools/hexa-sim/
// 현재는 스텁: 디렉터리 경로만 출력. TODO: 벤치러너 구현.

pub fn run(args: &[String]) -> Result<(), String> {
    let dir = super::tools_dir("hexa-sim");
    if args.is_empty() {
        println!("[hexa-sim] 자산 디렉터리: {}", dir);
        println!("  analysis/  benchmarks/  configs/");
        println!("  (TODO) 벤치 실행기 Rust 이식");
        return Ok(());
    }
    Err(format!(
        "[hexa-sim] 서브커맨드 미구현: {:?} — 디렉터리={}",
        args, dir
    ))
}
