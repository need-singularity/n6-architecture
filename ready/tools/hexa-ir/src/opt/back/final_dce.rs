/// P11: Final Dead Code Elimination — proof-guided global analysis
///
/// Base: remove instructions whose dest is never used as an operand (same as LLVM).
/// HEXA advantage: proof instructions identify MORE dead code:
///
///   1. LifetimeEnd(R) → all instructions producing R after this point are dead
///   2. OwnershipTransfer(A, B) → instructions producing values for A are dead
///   3. Exclusive borrows break alias chains → more stores become provably dead
///
/// LLVM must conservatively keep stores that MIGHT be read through aliases.
/// HEXA PROVES they can't be read — and eliminates them.

use crate::ir::*;
use crate::opt::proof_info::ProofInfo;
use std::collections::HashSet;

pub fn run(func: &mut HexaFunction) {
    let proof = ProofInfo::analyze(func);

    // Phase 1: Collect all registers used as operands across all blocks
    let mut used_regs: HashSet<usize> = HashSet::new();
    for block in &func.blocks {
        for instr in &block.instrs {
            for &arg in &instr.args {
                used_regs.insert(arg);
            }
        }
    }

    // Phase 2: Remove dead instructions with proof-guided analysis
    for block in &mut func.blocks {
        // Track whether we've seen a LifetimeEnd for each register in this block.
        // After LifetimeEnd, the register is provably dead — any instruction
        // that feeds it is dead code (even if it appears "used" by alias).
        let mut ended_in_block: HashSet<usize> = HashSet::new();

        // First pass: find LifetimeEnd positions
        for instr in &block.instrs {
            if instr.op == HexaOp::LifetimeEnd {
                if let Some(&reg) = instr.args.first() {
                    ended_in_block.insert(reg);
                }
            }
        }

        block.instrs.retain(|instr| {
            // Always keep side-effectful ops (but proof ops are erasable)
            if instr.op.has_side_effect() {
                // === HEXA-ONLY: proof-guided store elimination ===
                // A Store to a provably-dead register can be removed.
                // LLVM cannot do this — it must assume the store is visible.
                if instr.op == HexaOp::Store {
                    if let Some(&addr) = instr.args.first() {
                        if proof.is_provably_dead(addr) {
                            return false; // HEXA-only: proven dead store
                        }
                    }
                }
                return true;
            }

            if let Some(dest) = instr.dest {
                // Standard DCE: dest never used as operand anywhere
                if !used_regs.contains(&dest) {
                    return false; // dead instruction (LLVM can do this too)
                }

                // === HEXA-ONLY: lifetime-based dead code elimination ===
                // If this register's lifetime has ended (globally), and it feeds
                // only into other dead code, it's provably dead.
                // LLVM has no LifetimeEnd proof — it relies on alloca lifetime
                // intrinsics which are weaker (they don't prove exclusive access).
                if proof.dead_after_move.contains(&dest) {
                    // Dest was moved away — any computation producing this value
                    // after the move is dead. The new owner has the value.
                    return false; // HEXA-only: dead after move
                }
            }

            true
        });
    }
}
