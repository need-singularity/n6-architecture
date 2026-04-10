// special-number-bf — HEXA 분석 도구 래퍼
// 원본: tools/special-number-bf/main.hexa
// TODO: 순수 Rust 재구현

pub fn run(args: &[String]) -> Result<(), String> {
    super::hx("special-number-bf", args)
}
