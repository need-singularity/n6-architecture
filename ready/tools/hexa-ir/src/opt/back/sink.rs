/// P9: Code Sinking — move computations closer to use sites
///
/// Also removes duplicate LifetimeEnd markers and redundant
/// barrier instructions that block parallelism extraction.

use crate::ir::*;
use std::collections::HashSet;

pub fn run(func: &mut HexaFunction) {
    // Phase 1: Remove duplicate LifetimeEnd for the same register
    for block in &mut func.blocks {
        let mut ended: HashSet<usize> = HashSet::new();
        let mut dead_indices: Vec<bool> = vec![false; block.instrs.len()];

        for (i, instr) in block.instrs.iter().enumerate() {
            if instr.op == HexaOp::LifetimeEnd {
                if let Some(&reg) = instr.args.first() {
                    if !ended.insert(reg) {
                        dead_indices[i] = true; // duplicate lifetime end
                    }
                }
            }
        }

        let mut idx = 0;
        block.instrs.retain(|_| {
            let keep = !dead_indices[idx];
            idx += 1;
            keep
        });
    }

    // Phase 2: Sink pure computations that are only used in a single successor block
    // Mk.I: identify pure instructions in blocks with exactly one successor
    // whose results are only used in that successor. Move them there.
    let num_blocks = func.blocks.len();
    if num_blocks < 2 {
        return;
    }

    // Build a global use-map: register -> set of block IDs that use it
    let mut reg_use_blocks: std::collections::HashMap<usize, HashSet<usize>> =
        std::collections::HashMap::new();
    for block in &func.blocks {
        for instr in &block.instrs {
            for &arg in &instr.args {
                reg_use_blocks.entry(arg).or_default().insert(block.id);
            }
        }
    }

    // For each block with exactly one successor, try to sink pure instrs
    // that are only used in that successor
    for bi in 0..num_blocks {
        if func.blocks[bi].successors.len() != 1 {
            continue;
        }
        let succ_id = func.blocks[bi].successors[0];

        // Find the successor's index
        let succ_idx = func.blocks.iter().position(|b| b.id == succ_id);
        if succ_idx.is_none() {
            continue;
        }
        let succ_idx = succ_idx.unwrap();
        if succ_idx == bi {
            continue; // self-loop, don't sink
        }

        // Collect instructions to sink: pure, dest only used in successor
        let mut to_sink = Vec::new();
        let mut keep_indices: Vec<bool> = vec![true; func.blocks[bi].instrs.len()];

        for (i, instr) in func.blocks[bi].instrs.iter().enumerate() {
            if instr.op.has_side_effect() || instr.op.is_terminator() {
                continue;
            }
            if let Some(dest) = instr.dest {
                if let Some(use_blocks) = reg_use_blocks.get(&dest) {
                    // Only used in the successor block (not in current block or others)
                    if use_blocks.len() == 1 && use_blocks.contains(&succ_id) {
                        to_sink.push(instr.clone());
                        keep_indices[i] = false;
                    }
                }
            }
        }

        if !to_sink.is_empty() {
            // Remove from source block
            let mut idx = 0;
            func.blocks[bi].instrs.retain(|_| {
                let keep = keep_indices[idx];
                idx += 1;
                keep
            });

            // Prepend to successor block
            let mut new_instrs = to_sink;
            new_instrs.append(&mut func.blocks[succ_idx].instrs);
            func.blocks[succ_idx].instrs = new_instrs;
        }
    }
}
