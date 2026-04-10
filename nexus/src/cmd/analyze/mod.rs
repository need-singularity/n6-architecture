// analyze 카테고리 — 분석기/검증기/마이너 (HEXA 래퍼)

use crate::cmd::run_hexa_calc;

pub mod deep_miner;
pub mod formula_miner;
pub mod discovery_engine;
pub mod atlas_verifier;
pub mod hypothesis_grader;
pub mod lens_coverage;
pub mod bt_extension_verifier;
pub mod n6_discriminant;
pub mod reality_map_grow;
pub mod special_number_bf;
pub mod vendor_compare;
pub mod fusion_verify;

pub const NAMES: &[&str] = &[
    "deep-miner", "formula-miner", "discovery-engine", "atlas-verifier",
    "hypothesis-grader", "lens-coverage", "bt-extension-verifier", "n6-discriminant",
    "reality-map-grow", "special-number-bf", "vendor-compare", "fusion-verify",
];

pub fn run(name: &str, args: &[String]) -> Result<(), String> {
    match name {
        "deep-miner" | "deep_miner" => deep_miner::run(args),
        "formula-miner" | "formula_miner" => formula_miner::run(args),
        "discovery-engine" | "discovery_engine" => discovery_engine::run(args),
        "atlas-verifier" | "atlas_verifier" => atlas_verifier::run(args),
        "hypothesis-grader" | "hypothesis_grader" => hypothesis_grader::run(args),
        "lens-coverage" | "lens_coverage" => lens_coverage::run(args),
        "bt-extension-verifier" | "bt_extension_verifier" => bt_extension_verifier::run(args),
        "n6-discriminant" | "n6_discriminant" => n6_discriminant::run(args),
        "reality-map-grow" | "reality_map_grow" => reality_map_grow::run(args),
        "special-number-bf" | "special_number_bf" => special_number_bf::run(args),
        "vendor-compare" | "vendor_compare" | "vendor-compare-calc" => vendor_compare::run(args),
        "fusion-verify" | "fusion_verify" => fusion_verify::run(args),
        _ => Err(format!("analyze 하위 이름을 찾지 못함: {}", name)),
    }
}

pub(crate) fn hx(name: &str, args: &[String]) -> Result<(), String> {
    run_hexa_calc(name, args)
}
