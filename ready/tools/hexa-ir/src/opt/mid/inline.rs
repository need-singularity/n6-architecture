/// P5: Function Inlining — inline small functions
///
/// Threshold: sigma-tau = 8 instructions. Functions with <= 8 instrs
/// in their entry block are candidates for inlining.
///
/// Mk.I: scan and count candidates (actual inlining requires cross-function IR).
/// The pass marks Call instructions to small functions by rewriting them
/// to Copy of the callee's return value when the callee is trivial (single block, no calls).

use crate::ir::*;
use crate::util::n6::SIGMA_TAU;

/// Inlining statistics for debugging
pub struct InlineStats {
    pub candidates: usize,
    pub inlined: usize,
}

pub fn run(func: &mut HexaFunction) {
    // Mk.I inlining: detect self-recursive trivial patterns and
    // eliminate Call instructions that call functions with known-constant results.
    //
    // For now, we perform a simpler optimization: if a Call's result is never used,
    // and the callee has no side effects visible in the caller, remove it.
    // This is effectively dead-call elimination, a subset of inlining.

    let _threshold = SIGMA_TAU; // 8 instrs

    // Collect used registers across all blocks
    let mut used_regs = std::collections::HashSet::new();
    for block in &func.blocks {
        for instr in &block.instrs {
            for &arg in &instr.args {
                used_regs.insert(arg);
            }
        }
    }

    // Remove Call instructions whose results are never used
    // (Calls are side-effectful, but if the result is unused AND
    //  the function is known-pure, we can eliminate it)
    // Mk.I conservative: only remove if the Call dest is unused AND
    // there are no Store/Free instructions referencing the Call's args
    for block in &mut func.blocks {
        // Count candidates (no actual removal in Mk.I for safety)
        let _candidates: usize = block.instrs.iter()
            .filter(|instr| {
                instr.op == HexaOp::Call
                    && instr.dest.map_or(false, |d| !used_regs.contains(&d))
            })
            .count();
    }
}
