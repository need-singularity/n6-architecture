// dse 카테고리 — Design Space Exploration 도구 (HEXA 래퍼)

use crate::cmd::run_hexa_calc;

pub mod universal;
pub mod material;
pub mod solar;
pub mod fusion;
pub mod battery;
pub mod sc;
pub mod robot;
pub mod dse_calc;

pub const NAMES: &[&str] = &[
    "universal", "material", "solar", "fusion", "battery", "sc", "robot", "dse-calc",
];

pub fn run(name: &str, args: &[String]) -> Result<(), String> {
    match name {
        "universal" | "universal-dse" => universal::run(args),
        "material" | "material-dse" => material::run(args),
        "solar" | "solar-dse" => solar::run(args),
        "fusion" | "fusion-dse" => fusion::run(args),
        "battery" | "battery-dse" => battery::run(args),
        "sc" | "sc-dse" => sc::run(args),
        "robot" | "robot-dse" => robot::run(args),
        "dse-calc" | "dse_calc" | "calc" => dse_calc::run(args),
        _ => Err(format!("dse 하위 이름을 찾지 못함: {}", name)),
    }
}

pub(crate) fn hx(name: &str, args: &[String]) -> Result<(), String> {
    run_hexa_calc(name, args)
}
