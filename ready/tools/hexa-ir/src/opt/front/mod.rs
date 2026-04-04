/// Front-end passes (P1-P4) — tau=4 passes for early cleanup
///
/// P1: Type Inference — resolve Any types via context propagation
/// P2: Ownership Proof — deduplicate BorrowCheck instructions
/// P3: Dead Store Elimination — remove stores overwritten before read
/// P4: Constant Folding — fold constants + eliminate redundant loads

pub mod type_infer;
pub mod ownership_proof;
pub mod dead_store;
pub mod const_fold;

use crate::ir::*;
use std::time::Instant;

pub fn run(func: &mut HexaFunction) -> Vec<PassResult> {
    let mut results = Vec::new();
    let passes: Vec<(&str, fn(&mut HexaFunction))> = vec![
        ("P1: Type Inference",  type_infer::run),
        ("P2: Ownership Proof", ownership_proof::run),
        ("P3: Dead Store Elim", dead_store::run),
        ("P4: Const Fold+Load", const_fold::run),
    ];

    for (name, pass_fn) in &passes {
        let before = func.count_instrs();
        let start = Instant::now();
        pass_fn(func);
        let time_us = start.elapsed().as_micros() as u64;
        let after = func.count_instrs();
        results.push(PassResult {
            name, group: "Front",
            instrs_before: before, instrs_after: after, time_us,
        });
    }
    results
}
