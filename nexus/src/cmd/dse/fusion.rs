// fusion-dse — HEXA DSE 도구 래퍼
// 원본: tools/fusion-dse/main.hexa
// TODO: 순수 Rust 재구현

pub fn run(args: &[String]) -> Result<(), String> {
    super::hx("fusion-dse", args)
}
