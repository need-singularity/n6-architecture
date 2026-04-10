// fusion-verify — HEXA 분석 도구 래퍼
// 원본: tools/fusion-verify/main.hexa
// TODO: 순수 Rust 재구현

pub fn run(args: &[String]) -> Result<(), String> {
    super::hx("fusion-verify", args)
}
