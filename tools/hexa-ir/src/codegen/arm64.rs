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

/// ARM64 12-bit immediate limit for add/sub instructions
const ARM64_IMM12_MAX: u32 = 4095;

/// Emit sub sp, sp, #imm — handles large immediates via movz/movk + sub
fn emit_sub_sp_bin(code: &mut Vec<u8>, imm: u32) {
    if imm == 0 { return; }
    if imm <= ARM64_IMM12_MAX {
        // sub sp, sp, #imm: 0xD1000000 | imm12<<10 | 31<<5 | 31
        emit(code, 0xd1000000u32 | (imm << 10) | (31 << 5) | 31);
    } else {
        // movz x16, #lo16
        emit_load_imm_bin(code, 16, imm as u64);
        // sub sp, sp, x16: 0xCB000000 | x16<<16 | sp<<5 | sp
        emit(code, 0xcb000000u32 | (16 << 16) | (31 << 5) | 31);
    }
}

/// Emit add sp, sp, #imm — handles large immediates via movz/movk + add
fn emit_add_sp_bin(code: &mut Vec<u8>, imm: u32) {
    if imm == 0 { return; }
    if imm <= ARM64_IMM12_MAX {
        // add sp, sp, #imm: 0x91000000 | imm12<<10 | 31<<5 | 31
        emit(code, 0x91000000u32 | (imm << 10) | (31 << 5) | 31);
    } else {
        // movz x16, #lo16
        emit_load_imm_bin(code, 16, imm as u64);
        // add sp, sp, x16: 0x8B000000 | x16<<16 | sp<<5 | sp
        emit(code, 0x8b000000u32 | (16 << 16) | (31 << 5) | 31);
    }
}

/// Emit add rd, sp, #imm — handles large immediates
fn emit_add_reg_sp_imm_bin(code: &mut Vec<u8>, rd: u32, imm: u32) {
    if imm == 0 {
        // mov rd, sp = add rd, sp, #0
        emit(code, 0x91000000u32 | (31 << 5) | rd);
    } else if imm <= ARM64_IMM12_MAX {
        emit(code, 0x91000000u32 | (imm << 10) | (31 << 5) | rd);
    } else {
        // Load imm into rd, then add rd, sp, rd
        emit_load_imm_bin(code, rd, imm as u64);
        emit(code, 0x8b000000u32 | (rd << 16) | (31 << 5) | rd);
    }
}

/// Emit movz/movk sequence to load a 64-bit immediate into register rd
fn emit_load_imm_bin(code: &mut Vec<u8>, rd: u32, imm: u64) {
    let lo16 = (imm & 0xFFFF) as u32;
    // movz x<rd>, #lo16
    emit(code, 0xd2800000u32 | (lo16 << 5) | rd);
    let hi16 = ((imm >> 16) & 0xFFFF) as u32;
    if hi16 != 0 {
        // movk x<rd>, #hi16, lsl #16
        emit(code, 0xf2a00000u32 | (hi16 << 5) | rd);
    }
    let hi32 = ((imm >> 32) & 0xFFFF) as u32;
    if hi32 != 0 {
        // movk x<rd>, #hi32, lsl #32
        emit(code, 0xf2c00000u32 | (hi32 << 5) | rd);
    }
    let hi48 = ((imm >> 48) & 0xFFFF) as u32;
    if hi48 != 0 {
        // movk x<rd>, #hi48, lsl #48
        emit(code, 0xf2e00000u32 | (hi48 << 5) | rd);
    }
}

/// Select ARM64 instructions for a function
/// Walks HEXA-IR blocks and emits real ARM64 machine code
pub fn select_function(func: &HexaFunction, alloc: &RegAlloc) -> Vec<u8> {
    let mut code = Vec::new();

    // Pre-scan: detect array allocations and compute stack frame layout
    // Array Alloc = Alloc with Array type; maps SSA reg -> stack offset
    let mut array_stack_offsets: std::collections::HashMap<usize, u32> = std::collections::HashMap::new();
    let mut stack_frame_size: u32 = 0;
    for block in &func.blocks {
        for instr in &block.instrs {
            if instr.op == HexaOp::Alloc {
                if let Some(dest) = instr.dest {
                    if matches!(instr.ty, HexaType::Array(_, _) | HexaType::Struct(_)) {
                        let size = instr.args.first().copied().unwrap_or(0) as u32;
                        array_stack_offsets.insert(dest, stack_frame_size);
                        stack_frame_size += size;
                    }
                }
            }
        }
    }
    // Align stack frame to 16 bytes (ARM64 ABI requirement)
    if stack_frame_size > 0 {
        stack_frame_size = (stack_frame_size + 15) & !15;
        // Prologue: sub sp, sp, #frame_size (handles >4095 via movz/movk)
        emit_sub_sp_bin(&mut code, stack_frame_size);
    }

    // Collect constant values from Alloc instructions (Alloc %imm = load immediate)
    let mut reg_constants: std::collections::HashMap<usize, u64> = std::collections::HashMap::new();
    for block in &func.blocks {
        for instr in &block.instrs {
            if instr.op == HexaOp::Alloc {
                if let (Some(dest), Some(&imm)) = (instr.dest, instr.args.first()) {
                    // Don't treat array allocs as simple constants
                    if !array_stack_offsets.contains_key(&dest) {
                        reg_constants.insert(dest, imm as u64);
                    }
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
                    if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                        let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                        // Resolve first operand (may be spilled constant -> load to x14)
                        let rn = if let Some(phys) = alloc.get_phys(a) {
                            phys as u32
                        } else if let Some(&imm) = reg_constants.get(&a) {
                            let imm16 = (imm as u32) & 0xFFFF;
                            emit(&mut code, 0xd2800000u32 | (imm16 << 5) | 14);
                            14u32
                        } else {
                            0u32
                        };
                        // Optimize: if second operand is a known constant, use add-immediate
                        if let Some(&imm) = reg_constants.get(&b) {
                            if imm <= 0xFFF {
                                // add x<rd>, x<rn>, #imm: 0x91000000 | imm12<<10 | rn<<5 | rd
                                emit(&mut code, 0x91000000u32 | ((imm as u32) << 10) | (rn << 5) | rd);
                            } else {
                                // Load large constant into scratch x14
                                let imm16 = (imm as u32) & 0xFFFF;
                                emit(&mut code, 0xd2800000u32 | (imm16 << 5) | 14);
                                emit(&mut code, 0x8b000000u32 | (14u32 << 16) | (rn << 5) | rd);
                            }
                        } else {
                            // add x<rd>, x<rn>, x<rm>: 0x8B000000 | rm<<16 | rn<<5 | rd
                            let rm = alloc.get_phys(b).unwrap_or(0) as u32;
                            emit(&mut code, 0x8b000000u32 | (rm << 16) | (rn << 5) | rd);
                        }
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
                        // Handle constant operands: load into scratch x14 if spilled
                        let rn = if let Some(&imm) = reg_constants.get(&a) {
                            if alloc.get_phys(a).is_none() {
                                // Load constant into x14 scratch
                                let imm16 = (imm as u32) & 0xFFFF;
                                emit(&mut code, 0xd2800000u32 | (imm16 << 5) | 14);
                                14u32
                            } else {
                                alloc.get_phys(a).unwrap() as u32
                            }
                        } else {
                            alloc.get_phys(a).unwrap_or(0) as u32
                        };
                        let rm = if let Some(&imm) = reg_constants.get(&b) {
                            if alloc.get_phys(b).is_none() {
                                let imm16 = (imm as u32) & 0xFFFF;
                                emit(&mut code, 0xd2800000u32 | (imm16 << 5) | 14);
                                14u32
                            } else {
                                alloc.get_phys(b).unwrap() as u32
                            }
                        } else {
                            alloc.get_phys(b).unwrap_or(0) as u32
                        };
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
                        // Resolve base register (may be spilled)
                        let rn = if let Some(phys) = alloc.get_phys(base) {
                            phys as u32
                        } else if let Some(&imm) = reg_constants.get(&base) {
                            // Base is a constant — load address into scratch x14
                            let imm16 = (imm as u32) & 0xFFFF;
                            emit(&mut code, 0xd2800000u32 | (imm16 << 5) | 14);
                            14u32
                        } else {
                            0u32
                        };
                        emit(&mut code, 0xf9400000u32 | (rn << 5) | rd);
                    }
                }
                HexaOp::Store => {
                    // str x<rt>, [x<rn>]: 0xF9000000 | rn<<5 | rt
                    // args: [address, value_to_store]
                    if let (Some(&addr), Some(&val)) = (instr.args.get(0), instr.args.get(1)) {
                        // Resolve address register (may be spilled)
                        let rn = if let Some(phys) = alloc.get_phys(addr) {
                            phys as u32
                        } else if let Some(&imm) = reg_constants.get(&addr) {
                            let imm16 = (imm as u32) & 0xFFFF;
                            emit(&mut code, 0xd2800000u32 | (imm16 << 5) | 14);
                            14u32
                        } else {
                            0u32
                        };
                        // Resolve value register (may be a constant)
                        let rt = if let Some(phys) = alloc.get_phys(val) {
                            phys as u32
                        } else if let Some(&imm) = reg_constants.get(&val) {
                            // Load constant value into scratch x14 (or x13 if addr uses x14)
                            let scratch = if rn == 14 { 13u32 } else { 14u32 };
                            let imm16 = (imm as u32) & 0xFFFF;
                            emit(&mut code, 0xd2800000u32 | (imm16 << 5) | scratch);
                            scratch
                        } else {
                            0u32
                        };
                        emit(&mut code, 0xf9000000u32 | (rn << 5) | rt);
                    }
                }
                HexaOp::Alloc => {
                    if let Some(dest) = instr.dest {
                        if let Some(&offset) = array_stack_offsets.get(&dest) {
                            // Array allocation: x<rd> = sp + offset (handles >4095)
                            let rd = alloc.get_phys(dest).unwrap_or(0) as u32;
                            emit_add_reg_sp_imm_bin(&mut code, rd, offset);
                        } else if reg_constants.contains_key(&dest) {
                            // Scalar constant: only emit movz if it has a physical register
                            // (constants used only as immediates in Add/Mul don't need a register)
                            if let Some(phys) = alloc.get_phys(dest) {
                                if let Some(&imm) = instr.args.first() {
                                    let rd = phys as u32;
                                    let imm16 = (imm as u32) & 0xFFFF;
                                    emit(&mut code, 0xd2800000u32 | (imm16 << 5) | rd);
                                }
                            }
                            // If spilled, the constant is in reg_constants and will be
                            // used as immediate when referenced by Add/Mul/etc.
                        } else {
                            // Non-constant alloc (e.g., variable slot with no init)
                            if let Some(&imm) = instr.args.first() {
                                if let Some(phys) = alloc.get_phys(dest) {
                                    let rd = phys as u32;
                                    let imm16 = (imm as u32) & 0xFFFF;
                                    emit(&mut code, 0xd2800000u32 | (imm16 << 5) | rd);
                                }
                            }
                        }
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
                    // Epilogue: restore stack frame if arrays were allocated
                    if stack_frame_size > 0 {
                        // add sp, sp, #frame_size (handles >4095 via movz/movk)
                        emit_add_sp_bin(&mut code, stack_frame_size);
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

    // Epilogue: restore stack frame if arrays were allocated (for fallthrough paths)
    if stack_frame_size > 0 {
        emit_add_sp_bin(&mut code, stack_frame_size);
    }

    // For standalone executables: exit syscall instead of ret
    // x0 already holds return value (set by Return codegen above)
    // mov x16, #1 (SYS_exit on macOS)
    emit(&mut code, 0xd2800030u32); // movz x16, #1
    // svc #0x80
    emit(&mut code, 0xd4001001u32); // svc #0x80

    code
}
