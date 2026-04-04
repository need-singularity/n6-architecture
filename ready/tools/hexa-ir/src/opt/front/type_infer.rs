/// P1: Type Inference — resolve Any-typed instructions via context propagation
///
/// Hindley-Milner style: propagate concrete types from operands to results.
/// Arithmetic/comparison ops default to I64 if still unresolved after propagation.

use crate::ir::*;
use std::collections::HashMap;

pub fn run(func: &mut HexaFunction) {
    // Phase 1: Build register -> type map from already-typed instructions
    let mut reg_types: HashMap<usize, HexaType> = HashMap::new();
    for block in &func.blocks {
        for instr in &block.instrs {
            if let Some(dest) = instr.dest {
                if !matches!(instr.ty, HexaType::Any) {
                    reg_types.insert(dest, instr.ty.clone());
                }
            }
        }
    }

    // Phase 2: Resolve Any-typed instructions from their operands' known types
    // Iterate until fixpoint (multi-pass propagation)
    let mut changed = true;
    while changed {
        changed = false;
        for block in &mut func.blocks {
            for instr in &mut block.instrs {
                if !matches!(instr.ty, HexaType::Any) {
                    continue;
                }

                // Infer type from first typed argument
                let mut resolved = false;
                for &arg in &instr.args {
                    if let Some(ty) = reg_types.get(&arg) {
                        instr.ty = ty.clone();
                        if let Some(dest) = instr.dest {
                            reg_types.insert(dest, ty.clone());
                        }
                        resolved = true;
                        changed = true;
                        break;
                    }
                }

                // Arithmetic/comparison ops default to I64 if still unresolved
                if !resolved {
                    match instr.op {
                        HexaOp::Add | HexaOp::Sub | HexaOp::Mul |
                        HexaOp::Div | HexaOp::Mod | HexaOp::Neg => {
                            instr.ty = HexaType::I64;
                            if let Some(dest) = instr.dest {
                                reg_types.insert(dest, HexaType::I64);
                            }
                            changed = true;
                        }
                        HexaOp::Load => {
                            // Loads inherit type from their address's pointee type
                            // Default to I64 for unknown addresses
                            instr.ty = HexaType::I64;
                            if let Some(dest) = instr.dest {
                                reg_types.insert(dest, HexaType::I64);
                            }
                            changed = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
