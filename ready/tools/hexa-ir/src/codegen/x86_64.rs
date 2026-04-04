/// x86-64 Instruction Selection
///
/// Maps J2=24 HexaOps to x86-64 machine code bytes.
/// Uses SystemV ABI: first n=6 args in rdi, rsi, rdx, rcx, r8, r9.

use crate::ir::*;
use super::regalloc::{RegAlloc, PhysReg};

/// A single x86-64 machine instruction
#[derive(Clone, Debug)]
pub struct X86Instr {
    pub bytes: Vec<u8>,
    pub comment: String,
}

/// Select machine instructions for an entire function
pub fn select_function(func: &HexaFunction, alloc: &RegAlloc) -> Vec<u8> {
    let mut code = Vec::new();

    // Function prologue: push rbp; mov rbp, rsp; sub rsp, frame_size
    code.extend_from_slice(&[0x55]);                       // push rbp
    code.extend_from_slice(&[0x48, 0x89, 0xe5]);           // mov rbp, rsp
    if alloc.frame_size > 0 {
        // sub rsp, imm32
        code.extend_from_slice(&[0x48, 0x81, 0xec]);
        code.extend_from_slice(&(alloc.frame_size as u32).to_le_bytes());
    }

    // Emit instructions for each block
    for block in &func.blocks {
        for instr in &block.instrs {
            let dest_preg = instr.dest.and_then(|d| alloc.assignments.get(&d).copied());
            let arg_pregs: Vec<Option<PhysReg>> = instr.args.iter()
                .map(|&a| alloc.assignments.get(&a).copied())
                .collect();

            let x86 = select_instruction(&instr.op, dest_preg, &arg_pregs);
            code.extend_from_slice(&x86.bytes);
        }
    }

    // Function epilogue: mov rsp, rbp; pop rbp; ret
    code.extend_from_slice(&[0x48, 0x89, 0xec]);           // mov rsp, rbp
    code.extend_from_slice(&[0x5d]);                       // pop rbp
    code.extend_from_slice(&[0xc3]);                       // ret

    code
}

/// Select x86-64 instructions for a single HexaOp
fn select_instruction(
    op: &HexaOp,
    dest: Option<PhysReg>,
    args: &[Option<PhysReg>],
) -> X86Instr {
    let dest_reg = dest.unwrap_or(PhysReg::Rax);
    let arg0 = args.first().copied().flatten().unwrap_or(PhysReg::Rax);
    let arg1 = args.get(1).copied().flatten().unwrap_or(PhysReg::Rbx);

    match op {
        // ── Arithmetic ──
        HexaOp::Add => {
            // add dest, arg1  (assume dest already holds arg0 via mov)
            let mut bytes = Vec::new();
            emit_mov_reg_reg(&mut bytes, dest_reg, arg0);
            emit_alu_reg_reg(&mut bytes, 0x01, arg1, dest_reg); // ADD r/m64, r64
            X86Instr { bytes, comment: format!("add {:?}, {:?}", dest_reg, arg1) }
        }
        HexaOp::Sub => {
            let mut bytes = Vec::new();
            emit_mov_reg_reg(&mut bytes, dest_reg, arg0);
            emit_alu_reg_reg(&mut bytes, 0x29, arg1, dest_reg); // SUB r/m64, r64
            X86Instr { bytes, comment: format!("sub {:?}, {:?}", dest_reg, arg1) }
        }
        HexaOp::Mul => {
            // imul dest, arg1
            let mut bytes = Vec::new();
            emit_mov_reg_reg(&mut bytes, dest_reg, arg0);
            emit_imul_reg_reg(&mut bytes, dest_reg, arg1);
            X86Instr { bytes, comment: format!("imul {:?}, {:?}", dest_reg, arg1) }
        }
        HexaOp::Div => {
            // idiv: rax = rdx:rax / src, remainder in rdx
            let mut bytes = Vec::new();
            emit_mov_reg_reg(&mut bytes, PhysReg::Rax, arg0);
            bytes.extend_from_slice(&[0x48, 0x99]);            // cqo (sign-extend rax -> rdx:rax)
            emit_div_reg(&mut bytes, arg1);                     // idiv arg1
            if dest_reg != PhysReg::Rax {
                emit_mov_reg_reg(&mut bytes, dest_reg, PhysReg::Rax);
            }
            X86Instr { bytes, comment: format!("idiv {:?}", arg1) }
        }
        HexaOp::Mod => {
            // idiv, result in rdx
            let mut bytes = Vec::new();
            emit_mov_reg_reg(&mut bytes, PhysReg::Rax, arg0);
            bytes.extend_from_slice(&[0x48, 0x99]);            // cqo
            emit_div_reg(&mut bytes, arg1);
            if dest_reg != PhysReg::Rdx {
                emit_mov_reg_reg(&mut bytes, dest_reg, PhysReg::Rdx);
            }
            X86Instr { bytes, comment: format!("mod {:?}", arg1) }
        }
        HexaOp::Neg => {
            let mut bytes = Vec::new();
            emit_mov_reg_reg(&mut bytes, dest_reg, arg0);
            // neg r64: REX.W + F7 /3
            let rex = 0x48 | if dest_reg.needs_rex() { 0x01 } else { 0 };
            bytes.push(rex);
            bytes.push(0xf7);
            bytes.push(0xd8 | dest_reg.encoding()); // ModRM: /3 + reg
            X86Instr { bytes, comment: format!("neg {:?}", dest_reg) }
        }

        // ── Memory ──
        HexaOp::Load => {
            // mov dest, [arg0]
            let mut bytes = Vec::new();
            let rex = 0x48 | if dest_reg.needs_rex() { 0x04 } else { 0 }
                           | if arg0.needs_rex() { 0x01 } else { 0 };
            bytes.push(rex);
            bytes.push(0x8b); // MOV r64, r/m64
            bytes.push((dest_reg.encoding() << 3) | arg0.encoding()); // ModRM
            X86Instr { bytes, comment: format!("mov {:?}, [{:?}]", dest_reg, arg0) }
        }
        HexaOp::Store => {
            // mov [arg0], arg1
            let mut bytes = Vec::new();
            let rex = 0x48 | if arg1.needs_rex() { 0x04 } else { 0 }
                           | if arg0.needs_rex() { 0x01 } else { 0 };
            bytes.push(rex);
            bytes.push(0x89); // MOV r/m64, r64
            bytes.push((arg1.encoding() << 3) | arg0.encoding());
            X86Instr { bytes, comment: format!("mov [{:?}], {:?}", arg0, arg1) }
        }
        HexaOp::Alloc => {
            // sub rsp, 8; mov dest, rsp (stack alloc)
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&[0x48, 0x83, 0xec, 0x08]); // sub rsp, 8
            emit_mov_reg_reg(&mut bytes, dest_reg, PhysReg::Rsi); // placeholder
            // Actually: lea dest, [rsp]
            let rex = 0x48 | if dest_reg.needs_rex() { 0x04 } else { 0 };
            bytes.push(rex);
            bytes.push(0x8d); // LEA
            bytes.push((dest_reg.encoding() << 3) | 0x04); // ModRM: [rsp]
            bytes.push(0x24); // SIB: rsp base
            X86Instr { bytes, comment: format!("alloc {:?}", dest_reg) }
        }
        HexaOp::Free => {
            // add rsp, 8 (stack free)
            X86Instr {
                bytes: vec![0x48, 0x83, 0xc4, 0x08],
                comment: "free (add rsp, 8)".to_string(),
            }
        }
        HexaOp::Copy | HexaOp::Move => {
            let mut bytes = Vec::new();
            emit_mov_reg_reg(&mut bytes, dest_reg, arg0);
            X86Instr { bytes, comment: format!("mov {:?}, {:?}", dest_reg, arg0) }
        }

        // ── Control ──
        HexaOp::Jump => {
            // jmp rel32 (placeholder: 5-byte near jump)
            X86Instr {
                bytes: vec![0xe9, 0x00, 0x00, 0x00, 0x00],
                comment: "jmp <target>".to_string(),
            }
        }
        HexaOp::Branch => {
            // test arg0, arg0; jnz <then>; jmp <else>
            let mut bytes = Vec::new();
            let rex = 0x48 | if arg0.needs_rex() { 0x01 } else { 0 };
            bytes.push(rex);
            bytes.push(0x85); // TEST r/m64, r64
            bytes.push((arg0.encoding() << 3) | arg0.encoding());
            bytes.extend_from_slice(&[0x0f, 0x85, 0x00, 0x00, 0x00, 0x00]); // JNZ rel32
            bytes.extend_from_slice(&[0xe9, 0x00, 0x00, 0x00, 0x00]);       // JMP rel32
            X86Instr { bytes, comment: "branch".to_string() }
        }
        HexaOp::Call => {
            // SystemV ABI: args in rdi, rsi, rdx, rcx, r8, r9
            // call <target> (indirect via register)
            let mut bytes = Vec::new();
            // The first arg register holds the function pointer
            let rex = 0x40 | if arg0.needs_rex() { 0x01 } else { 0 };
            bytes.push(rex);
            bytes.push(0xff); // CALL r/m64
            bytes.push(0xd0 | arg0.encoding()); // ModRM: /2 + reg
            // Move result from rax to dest
            if dest_reg != PhysReg::Rax {
                emit_mov_reg_reg(&mut bytes, dest_reg, PhysReg::Rax);
            }
            X86Instr { bytes, comment: format!("call {:?}", arg0) }
        }
        HexaOp::Return => {
            let mut bytes = Vec::new();
            // Move return value to rax if needed
            if arg0 != PhysReg::Rax {
                emit_mov_reg_reg(&mut bytes, PhysReg::Rax, arg0);
            }
            // Epilogue + ret emitted at function level; just mark
            X86Instr { bytes, comment: "ret".to_string() }
        }
        HexaOp::Phi => {
            // Phi nodes are resolved during register allocation; no machine code
            X86Instr { bytes: vec![], comment: "phi (resolved)".to_string() }
        }
        HexaOp::Switch => {
            // Mk.I: emit as chain of cmp+je
            X86Instr {
                bytes: vec![0x90], // nop placeholder
                comment: "switch (stub)".to_string(),
            }
        }

        // ── Proof ops ── (no machine code; they're compile-time-only)
        HexaOp::ProofAssert | HexaOp::ProofInvariant | HexaOp::ProofWitness |
        HexaOp::OwnershipTransfer | HexaOp::BorrowCheck | HexaOp::LifetimeEnd => {
            X86Instr { bytes: vec![], comment: format!("{:?} (erased)", op) }
        }
    }
}

/// Emit REX.W + MOV r64, r64
fn emit_mov_reg_reg(bytes: &mut Vec<u8>, dest: PhysReg, src: PhysReg) {
    let rex = 0x48
        | if src.needs_rex() { 0x04 } else { 0 }
        | if dest.needs_rex() { 0x01 } else { 0 };
    bytes.push(rex);
    bytes.push(0x89); // MOV r/m64, r64
    bytes.push(0xc0 | (src.encoding() << 3) | dest.encoding());
}

/// Emit REX.W + ALU r/m64, r64
fn emit_alu_reg_reg(bytes: &mut Vec<u8>, opcode: u8, src: PhysReg, dest: PhysReg) {
    let rex = 0x48
        | if src.needs_rex() { 0x04 } else { 0 }
        | if dest.needs_rex() { 0x01 } else { 0 };
    bytes.push(rex);
    bytes.push(opcode);
    bytes.push(0xc0 | (src.encoding() << 3) | dest.encoding());
}

/// Emit REX.W + IMUL r64, r64
fn emit_imul_reg_reg(bytes: &mut Vec<u8>, dest: PhysReg, src: PhysReg) {
    let rex = 0x48
        | if dest.needs_rex() { 0x04 } else { 0 }
        | if src.needs_rex() { 0x01 } else { 0 };
    bytes.push(rex);
    bytes.push(0x0f);
    bytes.push(0xaf);
    bytes.push(0xc0 | (dest.encoding() << 3) | src.encoding());
}

/// Emit REX.W + IDIV r64
fn emit_div_reg(bytes: &mut Vec<u8>, src: PhysReg) {
    let rex = 0x48 | if src.needs_rex() { 0x01 } else { 0 };
    bytes.push(rex);
    bytes.push(0xf7);
    bytes.push(0xf8 | src.encoding()); // ModRM: /7 + reg
}
