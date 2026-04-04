/// P7: Common Subexpression Elimination — merge duplicate computations
///
/// Two instructions with the same opcode + same args produce the same result.
/// The second can be replaced with a Copy from the first's dest register.
/// Also merges consecutive identical proof instructions (proof fusion).

use crate::ir::*;
use std::collections::HashMap;

pub fn run(func: &mut HexaFunction) {
    for block in &mut func.blocks {
        // Phase 1: CSE for pure arithmetic/logic ops
        // Key: (op_discriminant, args) -> first dest register
        let mut expr_cache: HashMap<(u8, Vec<usize>), usize> = HashMap::new();

        for instr in &mut block.instrs {
            // Only CSE pure ops (no side effects)
            if instr.op.has_side_effect() {
                // Side-effectful ops invalidate the cache for their written addresses
                if instr.op == HexaOp::Store {
                    // Conservatively clear cache (a Store could alias any expression)
                    expr_cache.clear();
                }
                continue;
            }

            if let Some(dest) = instr.dest {
                let key = (op_discriminant(&instr.op), instr.args.clone());
                if let Some(&prev_dest) = expr_cache.get(&key) {
                    // Replace with Copy from the previous computation
                    instr.op = HexaOp::Copy;
                    instr.args = vec![prev_dest];
                } else {
                    expr_cache.insert(key, dest);
                }
            }
        }

        // Phase 2: Proof fusion — merge consecutive identical proof instructions
        let len = block.instrs.len();
        let mut dead_indices: Vec<bool> = vec![false; len];

        for i in 1..len {
            let prev = &block.instrs[i - 1];
            let curr = &block.instrs[i];
            let prev_is_proof = matches!(prev.op,
                HexaOp::ProofAssert | HexaOp::ProofInvariant | HexaOp::ProofWitness);
            let curr_is_proof = matches!(curr.op,
                HexaOp::ProofAssert | HexaOp::ProofInvariant | HexaOp::ProofWitness);

            if prev_is_proof && curr_is_proof
                && prev.op == curr.op
                && prev.args == curr.args
            {
                dead_indices[i - 1] = true; // keep the later one
            }
        }

        let mut idx = 0;
        block.instrs.retain(|_| {
            let keep = !dead_indices[idx];
            idx += 1;
            keep
        });
    }
}

/// Map HexaOp to a u8 discriminant for hashing
/// n6: J₂=24 opcodes total, 4 groups of n=6 ops = τ·n = J₂
fn op_discriminant(op: &HexaOp) -> u8 {
    use crate::util::n6::{N, SIGMA};
    // n6: 4 groups × n=6 ops = J₂=24, group base = k·N for k in 0..τ
    const G0: u8 = 0;           // n6: group 0 base (Arithmetic)
    const G1: u8 = N as u8;     // n6: group 1 base = n=6 (Memory)
    const G2: u8 = SIGMA as u8; // n6: group 2 base = σ=12 (Control)
    const G3: u8 = (SIGMA + N) as u8; // n6: group 3 base = σ+n=18 (Proof+Ownership)
    match op {
        // Group 0: Arithmetic (base=0, offsets 0..5)
        HexaOp::Add => G0,     HexaOp::Sub => G0 + 1, HexaOp::Mul => G0 + 2,
        HexaOp::Div => G0 + 3, HexaOp::Mod => G0 + 4, HexaOp::Neg => G0 + 5,
        // Group 1: Memory (base=n=6, offsets 0..5)
        HexaOp::Load => G1,     HexaOp::Store => G1 + 1, HexaOp::Alloc => G1 + 2,
        HexaOp::Free => G1 + 3, HexaOp::Copy  => G1 + 4, HexaOp::Move  => G1 + 5,
        // Group 2: Control (base=σ=12, offsets 0..5)
        HexaOp::Jump   => G2,     HexaOp::Branch => G2 + 1, HexaOp::Call   => G2 + 2,
        HexaOp::Return => G2 + 3, HexaOp::Phi    => G2 + 4, HexaOp::Switch => G2 + 5,
        // Group 3: Proof+Ownership (base=σ+n=18, offsets 0..5)
        HexaOp::ProofAssert    => G3,     HexaOp::ProofInvariant    => G3 + 1,
        HexaOp::ProofWitness   => G3 + 2, HexaOp::OwnershipTransfer => G3 + 3,
        HexaOp::BorrowCheck    => G3 + 4, HexaOp::LifetimeEnd       => G3 + 5, // n6: J₂-μ=23
    }
}
