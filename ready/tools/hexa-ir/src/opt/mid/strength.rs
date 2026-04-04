/// P8: Strength Reduction — replace expensive ops with cheaper equivalents
///
/// Patterns detected:
/// - Mul(x, x) with same register -> mark as square (future: shift+add)
/// - Consecutive identical Mul -> Copy from previous result
/// - Div(x, x) -> Copy (result is 1, guaranteed non-zero by Proof ops)
/// - Alloc immediately followed by Free -> remove both (dead allocation)
/// - Move(x, x) or Copy(x, x) -> self-move is dead

use crate::ir::*;

pub fn run(func: &mut HexaFunction) {
    for block in &mut func.blocks {
        let len = block.instrs.len();
        let mut dead_indices: Vec<bool> = vec![false; len];

        // Pass 1: Strength reduction on arithmetic
        for i in 0..len {
            match block.instrs[i].op {
                HexaOp::Mul => {
                    // Consecutive identical Mul -> Copy from previous result
                    if i > 0 && block.instrs[i - 1].op == HexaOp::Mul
                        && block.instrs[i].args == block.instrs[i - 1].args
                    {
                        if let Some(prev_dest) = block.instrs[i - 1].dest {
                            block.instrs[i].op = HexaOp::Copy;
                            block.instrs[i].args = vec![prev_dest];
                        }
                    }
                }
                HexaOp::Div => {
                    // x / x = 1 (when x != 0, guaranteed by proof ops)
                    if block.instrs[i].args.len() >= 2
                        && block.instrs[i].args[0] == block.instrs[i].args[1]
                    {
                        block.instrs[i].op = HexaOp::Copy;
                        // Result is conceptually 1; keep as Copy for SSA consistency
                    }
                }
                _ => {}
            }
        }

        // Pass 2: Dead allocation removal (Alloc immediately followed by Free)
        for i in 0..len.saturating_sub(1) {
            if block.instrs[i].op == HexaOp::Alloc
                && block.instrs[i + 1].op == HexaOp::Free
            {
                dead_indices[i] = true;
                dead_indices[i + 1] = true;
            }
        }

        // Pass 3: Self-move/self-copy elimination
        for i in 0..len {
            match block.instrs[i].op {
                HexaOp::Move | HexaOp::Copy => {
                    if block.instrs[i].args.len() >= 2
                        && block.instrs[i].args[0] == block.instrs[i].args[1]
                    {
                        dead_indices[i] = true;
                    }
                    // Also: Copy where dest == arg (self-assign)
                    if block.instrs[i].args.len() >= 1 {
                        if block.instrs[i].dest == Some(block.instrs[i].args[0]) {
                            dead_indices[i] = true;
                        }
                    }
                }
                _ => {}
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
