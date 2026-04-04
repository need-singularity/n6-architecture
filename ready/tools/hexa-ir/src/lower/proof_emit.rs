/// Proof instruction emission — insert ownership/borrow/lifetime IR ops
///
/// These proof instructions are unique to HEXA-IR (absent in Rust/LLVM).
/// They encode formal verification obligations directly in the IR,
/// enabling the optimizer to reason about safety without external analysis.

use crate::ir::*;
use super::LowerContext;

/// Emit an ownership transfer: `from_reg` gives ownership to `to_reg`
///
/// After this instruction, `from_reg` is conceptually dead (moved).
/// The optimizer can use this to eliminate redundant borrow checks
/// and the verifier (P12) can confirm no use-after-move.
pub fn emit_ownership_transfer(
    ctx: &mut LowerContext,
    block: &mut HexaBlock,
    from_reg: usize,
    to_reg: usize,
) {
    block.instrs.push(HexaInstr {
        op: HexaOp::OwnershipTransfer,
        dest: None,
        args: vec![from_reg, to_reg],
        ty: HexaType::Void,
            label: None,
    });
    // Emit a LifetimeEnd for the source register
    block.instrs.push(HexaInstr {
        op: HexaOp::LifetimeEnd,
        dest: None,
        args: vec![from_reg],
        ty: HexaType::Void,
            label: None,
    });
    let _ = ctx; // ctx available for future extensions
}

/// Emit a borrow check: assert that `reg` is currently borrowable
///
/// This is a runtime/compile-time assertion that the register
/// is still live and not moved. The optimizer (P2) deduplicates
/// redundant checks on the same register within a block.
pub fn emit_borrow_check(
    _ctx: &mut LowerContext,
    block: &mut HexaBlock,
    reg: usize,
) {
    block.instrs.push(HexaInstr {
        op: HexaOp::BorrowCheck,
        dest: None,
        args: vec![reg],
        ty: HexaType::Void,
            label: None,
    });
}

/// Emit a lifetime end marker: `reg` is no longer valid after this point
///
/// The optimizer uses this to:
/// - Free memory earlier (alloc/free pairing)
/// - Prove non-aliasing (two live ranges that don't overlap can share storage)
/// - Enable register coalescing in codegen
pub fn emit_lifetime_end(
    _ctx: &mut LowerContext,
    block: &mut HexaBlock,
    reg: usize,
) {
    block.instrs.push(HexaInstr {
        op: HexaOp::LifetimeEnd,
        dest: None,
        args: vec![reg],
        ty: HexaType::Void,
            label: None,
    });
}

/// Emit a proof assertion: the given condition register must be true
///
/// Used for assert!() expressions and compiler-generated safety checks.
/// The verifier (P12) ensures all ProofAssert conditions are satisfied.
pub fn emit_proof_assert(
    _ctx: &mut LowerContext,
    block: &mut HexaBlock,
    condition_reg: usize,
) {
    block.instrs.push(HexaInstr {
        op: HexaOp::ProofAssert,
        dest: None,
        args: vec![condition_reg],
        ty: HexaType::Void,
            label: None,
    });
}

/// Emit a proof invariant: `reg` satisfies the given loop/function invariant
///
/// Used at loop headers and function entry/exit to state properties
/// that must hold across iterations/calls.
pub fn emit_proof_invariant(
    _ctx: &mut LowerContext,
    block: &mut HexaBlock,
    reg: usize,
) {
    block.instrs.push(HexaInstr {
        op: HexaOp::ProofInvariant,
        dest: None,
        args: vec![reg],
        ty: HexaType::Void,
            label: None,
    });
}

/// Emit a proof witness: `reg` is a constructive proof of a type/property
///
/// Witnesses are used for type-level proofs (e.g., proving array bounds,
/// non-null guarantees). They carry no runtime cost but enable the
/// verifier to confirm safety statically.
pub fn emit_proof_witness(
    _ctx: &mut LowerContext,
    block: &mut HexaBlock,
    reg: usize,
) {
    block.instrs.push(HexaInstr {
        op: HexaOp::ProofWitness,
        dest: None,
        args: vec![reg],
        ty: HexaType::Void,
            label: None,
    });
}
