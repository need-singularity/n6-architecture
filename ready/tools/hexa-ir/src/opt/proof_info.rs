/// Proof-Guided Analysis — extract optimization facts from proof instructions
///
/// This is HEXA's unique advantage over LLVM:
/// LLVM has NO proof system, so it must assume the worst case for aliasing,
/// bounds, overflow. HEXA-IR's proof instructions (ProofAssert, ProofInvariant,
/// ProofWitness, OwnershipTransfer, BorrowCheck, LifetimeEnd) provide
/// GUARANTEED information the optimizer can exploit.
///
/// ProofInfo is computed once and shared across passes P3, P6, P11.
/// Proof instructions are erased at codegen — zero runtime cost.

use crate::ir::*;
use std::collections::{HashMap, HashSet};

/// Facts extracted from proof instructions for a single function
#[derive(Clone, Debug, Default)]
pub struct ProofInfo {
    /// Registers that have been ownership-transferred (from -> to).
    /// After transfer, `from` is provably dead — NO alias possible.
    /// LLVM can never know this without an ownership model.
    pub transfers: Vec<(usize, usize)>,

    /// Set of registers provably dead after ownership transfer.
    /// Any store to these addresses is dead (no one can read them).
    pub dead_after_move: HashSet<usize>,

    /// Registers with verified exclusive borrows.
    /// No other reference can alias these — enables store forwarding.
    pub exclusive_borrows: HashSet<usize>,

    /// Registers whose lifetime has ended at (block_id, instr_index).
    /// All uses after this point are provably impossible.
    pub lifetime_ended: HashMap<usize, (usize, usize)>,

    /// Loop headers with ProofInvariant-marked registers.
    /// These can be hoisted more aggressively than LICM's heuristic analysis.
    pub proven_loop_invariants: HashMap<usize, HashSet<usize>>,

    /// Registers with ProofAssert bounds (reg -> upper_bound).
    /// Array accesses through these need no bounds check.
    pub proven_bounds: HashMap<usize, usize>,

    /// Total proof instructions found (for metrics)
    pub proof_instr_count: usize,

    /// Optimizations enabled by proofs (for reporting)
    pub optimizations_enabled: usize,
}

impl ProofInfo {
    /// Extract all proof facts from a HexaFunction.
    /// This is a single-pass linear scan — O(n) in instruction count.
    pub fn analyze(func: &HexaFunction) -> Self {
        let mut info = ProofInfo::default();

        for block in &func.blocks {
            for (i, instr) in block.instrs.iter().enumerate() {
                match instr.op {
                    HexaOp::OwnershipTransfer => {
                        info.proof_instr_count += 1;
                        if instr.args.len() >= 2 {
                            let from = instr.args[0];
                            let to = instr.args[1];
                            info.transfers.push((from, to));
                            // After transfer, source register is provably dead.
                            // LLVM cannot deduce this — it has no ownership model.
                            info.dead_after_move.insert(from);
                        }
                    }

                    HexaOp::BorrowCheck => {
                        info.proof_instr_count += 1;
                        if let Some(&reg) = instr.args.first() {
                            // A verified BorrowCheck means exclusive access.
                            // No other pointer can alias this register.
                            // LLVM's alias analysis is purely heuristic — ours is proven.
                            info.exclusive_borrows.insert(reg);
                        }
                    }

                    HexaOp::LifetimeEnd => {
                        info.proof_instr_count += 1;
                        if let Some(&reg) = instr.args.first() {
                            // Register is provably dead after this point.
                            // Stores to it are dead. Uses are impossible.
                            // LLVM must conservatively keep potentially-aliased stores.
                            info.lifetime_ended.insert(reg, (block.id, i));
                        }
                    }

                    HexaOp::ProofInvariant => {
                        info.proof_instr_count += 1;
                        // ProofInvariant marks a register as loop-invariant.
                        // args[0] = register, and we associate it with the current block.
                        // This is stronger than LICM's heuristic: it's a PROOF.
                        if let Some(&reg) = instr.args.first() {
                            info.proven_loop_invariants
                                .entry(block.id)
                                .or_default()
                                .insert(reg);
                        }
                    }

                    HexaOp::ProofAssert => {
                        info.proof_instr_count += 1;
                        // ProofAssert(idx, len) — proves idx < len.
                        // Array access through idx needs no bounds check.
                        // LLVM inserts bounds checks or relies on UB — we have a proof.
                        if instr.args.len() >= 2 {
                            let idx_reg = instr.args[0];
                            let bound = instr.args[1];
                            info.proven_bounds.insert(idx_reg, bound);
                        }
                    }

                    HexaOp::ProofWitness => {
                        info.proof_instr_count += 1;
                        // ProofWitness carries auxiliary proof data.
                        // Currently used for verification; future passes may exploit it.
                    }

                    _ => {}
                }
            }
        }

        // Count optimizations this enables
        info.optimizations_enabled = info.dead_after_move.len()
            + info.exclusive_borrows.len()
            + info.lifetime_ended.len()
            + info.proven_loop_invariants.values().map(|s| s.len()).sum::<usize>()
            + info.proven_bounds.len();

        info
    }

    /// Check if two registers are provably non-aliasing.
    /// This is impossible for LLVM without restrict/noalias annotations.
    pub fn no_alias(&self, a: usize, b: usize) -> bool {
        // If A was transferred away, it can't alias anything
        if self.dead_after_move.contains(&a) || self.dead_after_move.contains(&b) {
            return true;
        }
        // If A has exclusive borrow, it can't alias B (and vice versa)
        if self.exclusive_borrows.contains(&a) && a != b {
            return true;
        }
        if self.exclusive_borrows.contains(&b) && a != b {
            return true;
        }
        false
    }

    /// Check if a register is provably dead (lifetime ended or moved away)
    pub fn is_provably_dead(&self, reg: usize) -> bool {
        self.dead_after_move.contains(&reg) || self.lifetime_ended.contains_key(&reg)
    }

    /// Check if a register is a proven loop invariant for the given block
    pub fn is_proven_invariant(&self, block_id: usize, reg: usize) -> bool {
        self.proven_loop_invariants
            .get(&block_id)
            .map_or(false, |regs| regs.contains(&reg))
    }
}
