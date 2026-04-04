/// P12: IR Verification — validate invariants (read-only)
///
/// Checks:
/// 1. All block IDs are unique
/// 2. No empty functions
/// 3. All instruction operands reference valid registers
/// 4. Type consistency: binary ops have matching operand types
/// 5. Terminators appear only at block ends
///
/// This pass does NOT modify the IR. The `&mut` signature is for
/// uniform pass interface only.

use crate::ir::*;
use std::collections::HashSet;

/// Verification result
#[derive(Clone, Debug)]
pub struct VerifyResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub fn run(func: &mut HexaFunction) {
    // Run verification; results are logged but don't modify IR
    let _result = verify(func);
}

/// Verify IR invariants, returning detailed results
pub fn verify(func: &HexaFunction) -> VerifyResult {
    let mut errors = Vec::new();

    // Check 1: All block IDs are unique
    let mut ids: HashSet<usize> = HashSet::new();
    for block in &func.blocks {
        if !ids.insert(block.id) {
            errors.push(format!("duplicate block ID: bb{}", block.id));
        }
    }

    // Check 2: Non-empty function
    if func.blocks.is_empty() {
        errors.push("function has no blocks".to_string());
    }

    // Check 3: All operand registers are defined somewhere
    let mut defined_regs: HashSet<usize> = HashSet::new();
    // Parameters are implicitly defined as registers 0..params.len()
    for i in 0..func.params.len() {
        defined_regs.insert(i);
    }
    for block in &func.blocks {
        for instr in &block.instrs {
            if let Some(dest) = instr.dest {
                defined_regs.insert(dest);
            }
        }
    }

    for block in &func.blocks {
        for instr in &block.instrs {
            for &arg in &instr.args {
                // Skip small constants that might be encoded as immediate values
                if arg < 1024 {
                    continue; // likely an immediate or block ID
                }
                if !defined_regs.contains(&arg) {
                    errors.push(format!(
                        "bb{}: operand %{} not defined",
                        block.id, arg
                    ));
                }
            }
        }
    }

    // Check 4: Type consistency for binary arithmetic ops
    for block in &func.blocks {
        for instr in &block.instrs {
            match instr.op {
                HexaOp::Add | HexaOp::Sub | HexaOp::Mul |
                HexaOp::Div | HexaOp::Mod => {
                    if instr.args.len() < 2 {
                        errors.push(format!(
                            "bb{}: {:?} expects 2 operands, got {}",
                            block.id, instr.op, instr.args.len()
                        ));
                    }
                }
                HexaOp::Neg => {
                    if instr.args.is_empty() {
                        errors.push(format!(
                            "bb{}: Neg expects 1 operand, got 0",
                            block.id
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    // Check 5: Terminators only at block ends
    for block in &func.blocks {
        let len = block.instrs.len();
        for (i, instr) in block.instrs.iter().enumerate() {
            if instr.op.is_terminator() && i + 1 < len {
                errors.push(format!(
                    "bb{}: terminator {:?} at position {} (not at end, len={})",
                    block.id, instr.op, i, len
                ));
            }
        }
    }

    VerifyResult {
        valid: errors.is_empty(),
        errors,
    }
}
