// lens-coverage — HEXA 분석 도구 래퍼
// 원본: tools/lens-coverage/main.hexa
// TODO: 순수 Rust 재구현

pub fn run(args: &[String]) -> Result<(), String> {
    super::hx("lens-coverage", args)
}
