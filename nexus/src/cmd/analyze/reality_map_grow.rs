// reality-map-grow — HEXA 분석 도구 래퍼
// 원본: tools/reality-map-grow/main.hexa
// TODO: 순수 Rust 재구현

pub fn run(args: &[String]) -> Result<(), String> {
    super::hx("reality-map-grow", args)
}
