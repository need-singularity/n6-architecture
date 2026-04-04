/// Mid-level passes (P5-P8) — tau=4 passes for core optimization
///
/// P5: Function Inlining — inline small functions (threshold sigma-tau=8 instrs)
/// P6: Loop Invariant Code Motion — hoist invariant loads out of loops
/// P7: Common Subexpression Elimination — merge duplicate computations
/// P8: Strength Reduction — replace expensive ops with cheaper equivalents

pub mod inline;
pub mod loop_opt;
pub mod cse;
pub mod strength;

use crate::ir::*;
use std::time::Instant;

pub fn run(func: &mut HexaFunction) -> Vec<PassResult> {
    let mut results = Vec::new();
    let passes: Vec<(&str, fn(&mut HexaFunction))> = vec![
        ("P5: Inline",         inline::run),
        ("P6: LICM",           loop_opt::run),
        ("P7: CSE",            cse::run),
        ("P8: Strength Red",   strength::run),
    ];

    for (name, pass_fn) in &passes {
        let before = func.count_instrs();
        let start = Instant::now();
        pass_fn(func);
        let time_us = start.elapsed().as_micros() as u64;
        let after = func.count_instrs();
        results.push(PassResult {
            name, group: "Mid",
            instrs_before: before, instrs_after: after, time_us,
        });
    }
    results
}
