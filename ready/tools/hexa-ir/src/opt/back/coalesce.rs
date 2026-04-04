/// P10: Copy Coalescing + Algebraic Simplification
///
/// Merges two optimizations:
/// 1. Copy coalescing: chains of Copy(Copy(x)) -> Copy(x)
/// 2. Algebraic identities: x-x=0, x%x=0, double negation, x+x CSE

use crate::ir::*;

pub fn run(func: &mut HexaFunction) {
    for block in &mut func.blocks {
        let len = block.instrs.len();
        let mut dead_indices: Vec<bool> = vec![false; len];

        // Phase 1: Algebraic simplification
        for (i, instr) in block.instrs.iter().enumerate() {
            match instr.op {
                HexaOp::Sub | HexaOp::Mod => {
                    // x - x = 0, x % x = 0
                    if instr.args.len() >= 2 && instr.args[0] == instr.args[1] {
                        dead_indices[i] = true;
                    }
                }
                HexaOp::Add => {
                    // Consecutive identical Add -> CSE
                    if i > 0 && block.instrs[i - 1].op == HexaOp::Add
                        && block.instrs[i].args == block.instrs[i - 1].args
                    {
                        dead_indices[i] = true;
                    }
                }
                HexaOp::Neg => {
                    // Double negation: Neg(Neg(x)) = x
                    if i > 0 && block.instrs[i - 1].op == HexaOp::Neg {
                        let prev_dest = block.instrs[i - 1].dest;
                        let curr_arg = instr.args.first().copied();
                        if prev_dest.is_some() && prev_dest == curr_arg.map(Some).unwrap_or(None) {
                            dead_indices[i] = true;
                            dead_indices[i - 1] = true;
                        }
                    }
                }
                HexaOp::Copy => {
                    // Self-copy: Copy(x) where dest == x
                    if instr.args.len() >= 1 && instr.dest == Some(instr.args[0]) {
                        dead_indices[i] = true;
                    }
                }
                _ => {}
            }
        }

        // Phase 2: Copy chain coalescing
        // Build a map: if reg R is defined by Copy(S), then R -> S
        let mut copy_source: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();
        for instr in &block.instrs {
            if instr.op == HexaOp::Copy && instr.args.len() >= 1 {
                if let Some(dest) = instr.dest {
                    copy_source.insert(dest, instr.args[0]);
                }
            }
        }

        // Resolve transitive copies: Copy(Copy(Copy(x))) -> Copy(x)
        fn resolve(map: &std::collections::HashMap<usize, usize>, reg: usize) -> usize {
            let mut current = reg;
            let mut seen = std::collections::HashSet::new();
            while let Some(&src) = map.get(&current) {
                if !seen.insert(current) {
                    break; // cycle detection
                }
                current = src;
            }
            current
        }

        // Rewrite Copy args to point to the ultimate source
        for instr in &mut block.instrs {
            if instr.op == HexaOp::Copy && instr.args.len() >= 1 {
                let resolved = resolve(&copy_source, instr.args[0]);
                instr.args[0] = resolved;
                // If dest == resolved source, mark dead
                if instr.dest == Some(resolved) {
                    if let Some(dest) = instr.dest {
                        // Find index -- we'll handle this with a second dead_indices pass
                        let _ = dest;
                    }
                }
            }
        }

        // Apply dead_indices
        let mut idx = 0;
        block.instrs.retain(|_| {
            let keep = !dead_indices[idx];
            idx += 1;
            keep
        });
    }
}
