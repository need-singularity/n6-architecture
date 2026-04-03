/// ARM64 (AArch64) Code Generation — Mk.II stub
///
/// ARM64 uses 31 general-purpose registers (x0-x30) + sp.
/// AAPCS64 ABI: first sigma-tau=8 args in x0-x7.

use crate::ir::*;
use super::regalloc::RegAlloc;

/// ARM64 physical register
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Arm64Reg {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30,
    Sp,
}

/// Select ARM64 instructions for a function
/// Walks HEXA-IR blocks and emits real ARM64 machine code
pub fn select_function(func: &HexaFunction, alloc: &RegAlloc) -> Vec<u8> {
    let mut code = Vec::new();

    // Minimal entry: no frame setup needed for leaf functions

    // Collect constant values from Alloc instructions (Alloc %imm = load immediate)
    let mut reg_constants: std::collections::HashMap<usize, u64> = std::collections::HashMap::new();
    for block in &func.blocks {
        for instr in &block.instrs {
            if instr.op == HexaOp::Alloc {
                if let (Some(dest), Some(&imm)) = (instr.dest, instr.args.first()) {
                    reg_constants.insert(dest, imm as u64);
                }
            }
        }
    }

    // Walk IR and emit instructions
    for block in &func.blocks {
        for instr in &block.instrs {
            match instr.op {
                HexaOp::Alloc => {
                    // Alloc %imm → mov x<dest>, #imm
                    if let (Some(dest), Some(&imm)) = (instr.dest, instr.args.first()) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let imm16 = (imm as u32) & 0xFFFF;
                        // movz x<rd>, #imm: 0xd2800000 | (imm16 << 5) | rd
                        let movz = 0xd2800000u32 | (imm16 << 5) | rd;
                        code.extend_from_slice(&movz.to_le_bytes());
                    }
                }
                HexaOp::Return => {
                    // Move return value to x0
                    if let Some(&arg) = instr.args.first() {
                        // Check if arg is a known constant
                        if let Some(&imm) = reg_constants.get(&arg) {
                            // mov x0, #imm directly
                            let imm16 = (imm as u32) & 0xFFFF;
                            let movz = 0xd2800000u32 | (imm16 << 5);
                            code.extend_from_slice(&movz.to_le_bytes());
                        } else {
                            let src = alloc.get_phys(arg).unwrap_or(0) as u32;
                            if src != 0 {
                                // mov x0, x<src>
                                let mov_instr = 0xaa0003e0u32 | (src << 16);
                                code.extend_from_slice(&mov_instr.to_le_bytes());
                            }
                        }
                    }
                }
                HexaOp::Add => {
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                        // add x<rd>, x<rn>, x<rm>: 0x8b000000 | rm<<16 | rn<<5 | rd
                        let add_instr = 0x8b000000u32 | (rm << 16) | (rn << 5) | rd;
                        code.extend_from_slice(&add_instr.to_le_bytes());
                    }
                }
                HexaOp::Sub => {
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                        // sub x<rd>, x<rn>, x<rm>
                        let sub_instr = 0xcb000000u32 | (rm << 16) | (rn << 5) | rd;
                        code.extend_from_slice(&sub_instr.to_le_bytes());
                    }
                }
                HexaOp::Mul => {
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                        // mul x<rd>, x<rn>, x<rm>: madd xd, xn, xm, xzr
                        let mul_instr = 0x9b007c00u32 | (rm << 16) | (rn << 5) | rd;
                        code.extend_from_slice(&mul_instr.to_le_bytes());
                    }
                }
                // Proof ops: zero runtime cost (erased)
                HexaOp::ProofAssert | HexaOp::ProofInvariant | HexaOp::ProofWitness |
                HexaOp::OwnershipTransfer | HexaOp::BorrowCheck | HexaOp::LifetimeEnd => {}
                // Other ops: TODO for full Mk.I
                _ => {}
            }
        }
    }

    // For standalone executables: exit syscall instead of ret
    // x0 already holds return value (set by Return codegen above)
    // mov x16, #1 (SYS_exit on macOS)
    code.extend_from_slice(&0xd2800030u32.to_le_bytes()); // movz x16, #1
    // svc #0x80
    code.extend_from_slice(&0xd4001001u32.to_le_bytes()); // svc #0x80

    code
}
