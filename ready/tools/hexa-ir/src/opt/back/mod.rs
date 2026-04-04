/// Back-end passes (P9-P12) — tau=4 passes for final cleanup + verification
///
/// P9:  Code Sinking — move computations closer to their use sites
/// P10: Copy Coalescing — merge algebraic identities + simplifications
/// P11: Final DCE — dead code elimination with global use-set analysis
/// P12: IR Verification — validate invariants (read-only, no modification)

pub mod sink;
pub mod coalesce;
pub mod final_dce;
pub mod verify;

use crate::ir::*;
use std::time::Instant;

pub fn run(func: &mut HexaFunction) -> Vec<PassResult> {
    let mut results = Vec::new();

    // P9-P11: mutable passes
    let passes: Vec<(&str, fn(&mut HexaFunction))> = vec![
        ("P9: Code Sinking",   sink::run),
        ("P10: Coalesce",      coalesce::run),
        ("P11: Final DCE",     final_dce::run),
    ];

    for (name, pass_fn) in &passes {
        let before = func.count_instrs();
        let start = Instant::now();
        pass_fn(func);
        let time_us = start.elapsed().as_micros() as u64;
        let after = func.count_instrs();
        results.push(PassResult {
            name, group: "Back",
            instrs_before: before, instrs_after: after, time_us,
        });
    }

    // P12: Verify (read-only pass — wrapped as mutable for uniform interface)
    let before = func.count_instrs();
    let start = Instant::now();
    verify::run(func);
    let time_us = start.elapsed().as_micros() as u64;
    results.push(PassResult {
        name: "P12: Verify", group: "Back",
        instrs_before: before, instrs_after: before, time_us,
    });

    results
}
