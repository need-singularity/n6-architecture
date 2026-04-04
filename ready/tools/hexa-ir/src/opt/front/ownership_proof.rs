/// P2: Ownership Proof — eliminate redundant borrow checks
///
/// Multiple BorrowCheck on the same register within a block -> keep only first.
/// Also deduplicate consecutive identical OwnershipTransfer instructions.

use crate::ir::*;
use std::collections::HashSet;

pub fn run(func: &mut HexaFunction) {
    for block in &mut func.blocks {
        let mut checked_regs: HashSet<usize> = HashSet::new();
        let mut transfer_pairs: HashSet<(usize, usize)> = HashSet::new();

        block.instrs.retain(|instr| {
            match instr.op {
                HexaOp::BorrowCheck => {
                    if let Some(&reg) = instr.args.first() {
                        if !checked_regs.insert(reg) {
                            return false; // duplicate borrow check
                        }
                    }
                }
                HexaOp::OwnershipTransfer => {
                    if instr.args.len() >= 2 {
                        let pair = (instr.args[0], instr.args[1]);
                        if !transfer_pairs.insert(pair) {
                            return false; // duplicate transfer
                        }
                    }
                }
                _ => {}
            }
            true
        });
    }
}
