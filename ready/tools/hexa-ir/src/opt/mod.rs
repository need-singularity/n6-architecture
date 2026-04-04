/// Optimization Pipeline — sigma=12 passes in n/phi=3 waves
///
/// Front (P1-P4): type resolution, ownership dedup, proof-guided DSE, const fold
/// Mid   (P5-P8): inlining, proof-guided LICM, CSE, strength reduction
/// Back  (P9-P12): sinking, coalescing, proof-guided DCE, verification
///
/// HEXA advantage over LLVM: passes P3, P6, P11 exploit proof instructions
/// (OwnershipTransfer, BorrowCheck, LifetimeEnd, ProofInvariant, ProofAssert)
/// to eliminate code that LLVM must conservatively keep. Zero runtime cost —
/// proof instructions are erased at codegen.

pub mod proof_info;
pub mod front;
pub mod mid;
pub mod back;

use crate::ir::*;

/// Run the complete sigma=12 optimization pipeline
pub fn run_pipeline(func: &mut HexaFunction) -> Vec<PassResult> {
    let mut results = Vec::new();
    results.extend(front::run(func));
    results.extend(mid::run(func));
    results.extend(back::run(func));
    results
}
