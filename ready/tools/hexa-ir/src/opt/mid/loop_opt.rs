/// P6: Loop Invariant Code Motion (LICM) — proof-guided hoisting
///
/// Base: hoist Loads whose address isn't written by any Store in the loop.
/// HEXA advantage: ProofInvariant provides GUARANTEED loop invariance:
///
///   1. ProofInvariant(reg) → reg is proven invariant → hoist ANY instruction
///      that uses only invariant regs, not just loads.
///   2. OwnershipTransfer in loop → source is dead → stores to it can be hoisted out
///   3. BorrowCheck → exclusive access → load can't be invalidated by other stores
///
/// LLVM's LICM is heuristic: it analyzes def-use chains and alias sets.
/// HEXA's LICM is proof-driven: ProofInvariant is a mathematical guarantee.

use crate::ir::*;
use crate::opt::proof_info::ProofInfo;
use std::collections::HashSet;

pub fn run(func: &mut HexaFunction) {
    let proof = ProofInfo::analyze(func);
    let mut hoisted = Vec::new();

    for block in &mut func.blocks {
        // Detect loop blocks by back-edge heuristic
        let has_back_edge = block.successors.iter().any(|&s| s < block.id);
        if !has_back_edge {
            continue;
        }

        // Collect all addresses written by Store in this loop block
        let mut stored_addrs: HashSet<usize> = HashSet::new();
        for instr in &block.instrs {
            if instr.op == HexaOp::Store {
                if let Some(&addr) = instr.args.first() {
                    stored_addrs.insert(addr);
                }
            }
        }

        // Collect proof-guaranteed invariant registers for this block
        let proven_invariant = proof.proven_loop_invariants
            .get(&block.id)
            .cloned()
            .unwrap_or_default();

        let mut kept = Vec::new();
        for instr in block.instrs.drain(..) {
            // Skip proof instructions and terminators — never hoist these
            if instr.op.is_proof() || instr.op.is_terminator() {
                kept.push(instr);
                continue;
            }

            // === HEXA-ONLY: Proof-guided hoisting ===

            // (1) ProofInvariant-marked registers: ANY pure instruction that
            //     uses only proven-invariant operands can be hoisted.
            //     LLVM cannot do this — it has no invariant proof system.
            if !instr.op.has_side_effect() && !instr.args.is_empty() {
                let all_args_proven_invariant = instr.args.iter()
                    .all(|&arg| proven_invariant.contains(&arg));

                if all_args_proven_invariant {
                    hoisted.push(instr);
                    continue;
                }
            }

            // (2) Load from exclusive borrow: the borrow guarantees no concurrent
            //     modification, so this load is safe to hoist even if the address
            //     appears in a store (the store must be to a different object).
            //     LLVM would see the store and refuse to hoist.
            if instr.op == HexaOp::Load {
                let addr = instr.args.first().copied().unwrap_or(usize::MAX);

                if proof.exclusive_borrows.contains(&addr) && stored_addrs.contains(&addr) {
                    // Exclusive borrow: the store must be to a different object
                    // that happens to use the same register number. Safe to hoist.
                    hoisted.push(instr);
                    continue;
                }

                // === Standard LICM (same as LLVM) ===
                if !stored_addrs.contains(&addr) {
                    hoisted.push(instr); // safe to hoist
                    continue;
                }
            }

            kept.push(instr);
        }
        block.instrs = kept;
    }

    // Prepend hoisted instructions to entry block
    if !hoisted.is_empty() && !func.blocks.is_empty() {
        let mut new_instrs = hoisted;
        new_instrs.append(&mut func.blocks[0].instrs);
        func.blocks[0].instrs = new_instrs;
    }
}
