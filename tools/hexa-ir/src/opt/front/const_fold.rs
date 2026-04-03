/// P4: Constant Folding + Redundant Load Elimination
///
/// Merges two optimizations:
/// 1. Constant folding: Alloc with known value args -> propagate through arithmetic
/// 2. Redundant loads: multiple Loads from same address (no intervening Store) -> reuse first

use crate::ir::*;
use std::collections::HashMap;

pub fn run(func: &mut HexaFunction) {
    // Phase 1: Redundant Load Elimination (CSE for loads)
    for block in &mut func.blocks {
        let mut load_cache: HashMap<usize, usize> = HashMap::new(); // addr -> first dest reg
        let mut dead_indices: Vec<bool> = vec![false; block.instrs.len()];
        let mut reg_remap: HashMap<usize, usize> = HashMap::new(); // dead dest -> reuse dest

        for (i, instr) in block.instrs.iter().enumerate() {
            if instr.op == HexaOp::Store {
                // Invalidate cache for stored address
                if let Some(&addr) = instr.args.first() {
                    load_cache.remove(&addr);
                }
            } else if instr.op == HexaOp::Load {
                if let Some(&addr) = instr.args.first() {
                    if let Some(&first_dest) = load_cache.get(&addr) {
                        // Redundant load: remap this dest to the first load's dest
                        if let Some(dest) = instr.dest {
                            reg_remap.insert(dest, first_dest);
                        }
                        dead_indices[i] = true; // redundant load
                    } else if let Some(dest) = instr.dest {
                        load_cache.insert(addr, dest);
                    }
                }
            }
        }

        // Rewrite all references to dead load dests to use the first load's dest
        if !reg_remap.is_empty() {
            for instr in &mut block.instrs {
                for arg in &mut instr.args {
                    if let Some(&new_reg) = reg_remap.get(arg) {
                        *arg = new_reg;
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

    // Phase 2: Constant propagation through Alloc -> arithmetic chains
    // Track registers defined by Alloc with a known constant in args[0]
    let mut const_regs: HashMap<usize, i64> = HashMap::new();
    for block in &func.blocks {
        for instr in &block.instrs {
            if instr.op == HexaOp::Alloc {
                if let Some(dest) = instr.dest {
                    if let Some(&val) = instr.args.first() {
                        if matches!(instr.ty, HexaType::I64 | HexaType::Bool) {
                            const_regs.insert(dest, val as i64);
                        }
                    }
                }
            }
        }
    }

    // Phase 3: Fold arithmetic on known constants
    for block in &mut func.blocks {
        for instr in &mut block.instrs {
            if instr.args.len() >= 2 {
                let lhs = instr.args[0];
                let rhs = instr.args[1];
                if let (Some(&lv), Some(&rv)) = (const_regs.get(&lhs), const_regs.get(&rhs)) {
                    let folded = match instr.op {
                        HexaOp::Add => Some(lv.wrapping_add(rv)),
                        HexaOp::Sub => Some(lv.wrapping_sub(rv)),
                        HexaOp::Mul => Some(lv.wrapping_mul(rv)),
                        HexaOp::Div if rv != 0 => Some(lv.wrapping_div(rv)),
                        HexaOp::Mod if rv != 0 => Some(lv.wrapping_rem(rv)),
                        _ => None,
                    };
                    if let Some(result) = folded {
                        if let Some(dest) = instr.dest {
                            // Replace with Alloc of the folded constant
                            instr.op = HexaOp::Alloc;
                            instr.args = vec![result as usize];
                            const_regs.insert(dest, result);
                        }
                    }
                }
            }
        }
    }
}
