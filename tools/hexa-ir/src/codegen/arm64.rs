/// ARM64 (AArch64) Code Generation — Mk.II full J₂=24 opcodes
///
/// ARM64 uses 31 general-purpose registers (x0-x30) + sp.
/// AAPCS64 ABI: first sigma-tau=8 args in x0-x7.
/// All J₂=24 HEXA-IR opcodes have explicit match arms (no catch-all).

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

/// Helper: emit a single 32-bit ARM64 instruction (little-endian)
#[inline]
fn emit(code: &mut Vec<u8>, instr: u32) {
    code.extend_from_slice(&instr.to_le_bytes());
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

    // Walk IR and emit instructions — all J₂=24 opcodes covered
    for block in &func.blocks {
        for instr in &block.instrs {
            match instr.op {
                // ── Arithmetic (n=6) ──

                HexaOp::Add => {
                    // add x<rd>, x<rn>, x<rm>: 0x8B000000 | rm<<16 | rn<<5 | rd
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                        emit(&mut code, 0x8b000000u32 | (rm << 16) | (rn << 5) | rd);
                    }
                }
                HexaOp::Sub => {
                    // sub x<rd>, x<rn>, x<rm>: 0xCB000000 | rm<<16 | rn<<5 | rd
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                        emit(&mut code, 0xcb000000u32 | (rm << 16) | (rn << 5) | rd);
                    }
                }
                HexaOp::Mul => {
                    // mul x<rd>, x<rn>, x<rm> = madd x<rd>, x<rn>, x<rm>, xzr
                    // 0x9B007C00 | rm<<16 | rn<<5 | rd
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                        emit(&mut code, 0x9b007c00u32 | (rm << 16) | (rn << 5) | rd);
                    }
                }
                HexaOp::Div => {
                    // sdiv x<rd>, x<rn>, x<rm>: 0x9AC00C00 | rm<<16 | rn<<5 | rd
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                        emit(&mut code, 0x9ac00c00u32 | (rm << 16) | (rn << 5) | rd);
                    }
                }
                HexaOp::Mod => {
                    // rd = rn % rm  →  tmp = sdiv(rn, rm); rd = msub(tmp, rm, rn)
                    // Uses x15 as scratch register for the intermediate quotient
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                        let tmp: u32 = 15; // x15 scratch
                        // sdiv x15, x<rn>, x<rm>
                        emit(&mut code, 0x9ac00c00u32 | (rm << 16) | (rn << 5) | tmp);
                        // msub x<rd>, x15, x<rm>, x<rn>  (rd = rn - x15*rm)
                        // msub: 0x9B008000 | rm<<16 | ra<<10 | rn_divisor<<5 | rd
                        // Encoding: 1_00_11011_000_Rm_1_Ra_Rn_Rd
                        emit(&mut code, 0x9b008000u32 | (rm << 16) | (rn << 10) | (tmp << 5) | rd);
                    }
                }
                HexaOp::Neg => {
                    // neg x<rd>, x<rn> = sub x<rd>, xzr, x<rn>
                    // 0xCB0003E0 | rn<<16 | rd
                    if let (Some(dest), Some(&a)) = (instr.dest, instr.args.first()) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(a).unwrap_or(0) as u32;
                        emit(&mut code, 0xcb0003e0u32 | (rn << 16) | rd);
                    }
                }

                // ── Memory (n=6) ──

                HexaOp::Load => {
                    // ldr x<rd>, [x<rn>]: 0xF9400000 | rn<<5 | rd
                    if let (Some(dest), Some(&base)) = (instr.dest, instr.args.first()) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(base).unwrap_or(0) as u32;
                        emit(&mut code, 0xf9400000u32 | (rn << 5) | rd);
                    }
                }
                HexaOp::Store => {
                    // str x<rs>, [x<rn>]: 0xF9000000 | rn<<5 | rs
                    // args: [value_to_store, address]
                    if let (Some(&val), Some(&addr)) = (instr.args.get(0), instr.args.get(1)) {
                        let rs = alloc.get_phys(val).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(addr).unwrap_or(0) as u32;
                        emit(&mut code, 0xf9000000u32 | (rn << 5) | rs);
                    }
                }
                HexaOp::Alloc => {
                    // Alloc %imm → movz x<dest>, #imm
                    if let (Some(dest), Some(&imm)) = (instr.dest, instr.args.first()) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let imm16 = (imm as u32) & 0xFFFF;
                        emit(&mut code, 0xd2800000u32 | (imm16 << 5) | rd);
                    }
                }
                HexaOp::Free => {
                    // No-op: memory deallocation handled by runtime allocator
                    // ARM64: nop = 0xD503201F
                }
                HexaOp::Copy => {
                    // mov x<rd>, x<rn> = orr x<rd>, xzr, x<rn>
                    // 0xAA0003E0 | rn<<16 | rd
                    if let (Some(dest), Some(&src)) = (instr.dest, instr.args.first()) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(src).unwrap_or(0) as u32;
                        emit(&mut code, 0xaa0003e0u32 | (rn << 16) | rd);
                    }
                }
                HexaOp::Move => {
                    // Same as Copy at machine level (source invalidation is a proof concern)
                    // mov x<rd>, x<rn> = orr x<rd>, xzr, x<rn>
                    if let (Some(dest), Some(&src)) = (instr.dest, instr.args.first()) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        let rn = alloc.get_phys(src).unwrap_or(0) as u32;
                        emit(&mut code, 0xaa0003e0u32 | (rn << 16) | rd);
                    }
                }

                // ── Control (n=6) ──

                HexaOp::Jump => {
                    // b <offset>: 0x14000000 | imm26
                    // Placeholder: b #0 (offset patched by linker/relocation)
                    emit(&mut code, 0x14000000u32);
                }
                HexaOp::Branch => {
                    // cbnz x<cond>, <offset>: 0xB5000000 | imm19<<5 | rt
                    // Simplified: branch on first arg != 0, offset placeholder
                    if let Some(&cond) = instr.args.first() {
                        let rt = alloc.get_phys(cond).unwrap_or(0) as u32;
                        emit(&mut code, 0xb5000000u32 | rt);
                    } else {
                        // Unconditional branch fallback
                        emit(&mut code, 0x14000000u32);
                    }
                }
                HexaOp::Call => {
                    // bl <offset>: 0x94000000 | imm26
                    // Placeholder: bl #0 (offset patched by linker/relocation)
                    emit(&mut code, 0x94000000u32);
                    // Result arrives in x0 per AAPCS64; move to dest if needed
                    if let Some(dest) = instr.dest {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        if rd != 0 {
                            // mov x<rd>, x0
                            emit(&mut code, 0xaa0003e0u32 | (0u32 << 16) | rd);
                        }
                    }
                }
                HexaOp::Return => {
                    // Move return value to x0
                    if let Some(&arg) = instr.args.first() {
                        if let Some(&imm) = reg_constants.get(&arg) {
                            // mov x0, #imm directly
                            let imm16 = (imm as u32) & 0xFFFF;
                            emit(&mut code, 0xd2800000u32 | (imm16 << 5));
                        } else {
                            let src = alloc.get_phys(arg).unwrap_or(0) as u32;
                            if src != 0 {
                                // mov x0, x<src>
                                emit(&mut code, 0xaa0003e0u32 | (src << 16));
                            }
                        }
                    }
                }
                HexaOp::Phi => {
                    // Phi nodes are resolved during register allocation — no machine code
                }
                HexaOp::Switch => {
                    // Emit as chain of cbnz branches (one per case)
                    // Each case: cmp x<selector>, #case_val; b.eq <target>
                    // Simplified: emit cbnz on selector as placeholder
                    if let Some(&sel) = instr.args.first() {
                        let rt = alloc.get_phys(sel).unwrap_or(0) as u32;
                        // For each additional arg (case targets), emit cbnz placeholder
                        let case_count = if instr.args.len() > 1 { instr.args.len() - 1 } else { 1 };
                        for _ in 0..case_count {
                            // cbnz x<sel>, #0 (offset patched later)
                            emit(&mut code, 0xb5000000u32 | rt);
                        }
                        // Default: unconditional branch placeholder
                        emit(&mut code, 0x14000000u32);
                    } else {
                        // No selector — emit unconditional branch
                        emit(&mut code, 0x14000000u32);
                    }
                }

                // ── Proof (n=6) — zero runtime cost, compile-time erased ──

                HexaOp::ProofAssert => {}
                HexaOp::ProofInvariant => {}
                HexaOp::ProofWitness => {}
                HexaOp::OwnershipTransfer => {}
                HexaOp::BorrowCheck => {}
                HexaOp::LifetimeEnd => {}
            }
        }
    }

    // For standalone executables: exit syscall instead of ret
    // x0 already holds return value (set by Return codegen above)
    // mov x16, #1 (SYS_exit on macOS)
    emit(&mut code, 0xd2800030u32); // movz x16, #1
    // svc #0x80
    emit(&mut code, 0xd4001001u32); // svc #0x80

    code
}
