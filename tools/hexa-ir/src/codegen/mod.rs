/// Code Generation — IR -> native machine code
///
/// Pipeline: regalloc -> instruction selection -> binary emission
/// Targets: x86-64 (Mk.I), ARM64 (Mk.II stub)
/// Output formats: ELF (Linux), Mach-O (macOS)

pub mod regalloc;
pub mod x86_64;
pub mod arm64;
pub mod elf;
pub mod macho;

use crate::ir::*;

/// Compilation target
#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    X86_64Linux,
    X86_64MacOS,
    Arm64MacOS,
}

impl Target {
    /// Detect native target for current platform
    pub fn native() -> Self {
        if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") { Target::Arm64MacOS }
            else { Target::X86_64MacOS }
        } else {
            Target::X86_64Linux
        }
    }
}

/// Compile multiple functions to a single binary
pub fn compile_to_binary(functions: &[HexaFunction], target: Target) -> Vec<u8> {
    let mut all_code = Vec::new();
    for func in functions {
        let code = generate(func, &target);
        all_code.extend_from_slice(&code);
    }
    all_code
}

/// Compile and link to a native executable using system linker
/// Returns the path to the output binary
pub fn compile_and_link(functions: &[HexaFunction], target: Target, output: &str) -> Result<(), String> {
    let asm_path = format!("{}.s", output);

    match &target {
        Target::Arm64MacOS => {
            // Use readable assembly emission for ARM64 — supports full IR
            let asm = arm64_asm::emit_program_asm(functions);
            std::fs::write(&asm_path, &asm).map_err(|e| format!("write asm: {}", e))?;
        }
        _ => {
            // x86-64: use .byte directive approach
            let mut all_code = Vec::new();
            for func in functions {
                let alloc = regalloc::allocate(func);
                let mc = match &target {
                    Target::X86_64Linux | Target::X86_64MacOS => x86_64::select_function(func, &alloc),
                    _ => unreachable!(),
                };
                all_code.extend_from_slice(&mc);
            }

            let mut asm = String::new();
            asm.push_str(".global _main\n_main:\n");
            for byte in &all_code {
                asm.push_str(&format!("    .byte 0x{:02x}\n", byte));
            }
            std::fs::write(&asm_path, &asm).map_err(|e| format!("write asm: {}", e))?;
        }
    }

    // Link with system cc
    let status = std::process::Command::new("cc")
        .args(&[&asm_path, "-o", output, "-lSystem", "-e", "_main"])
        .status()
        .map_err(|e| format!("cc: {}", e))?;

    // Clean up asm
    let _ = std::fs::remove_file(&asm_path);

    if status.success() {
        Ok(())
    } else {
        Err(format!("linker failed with {}", status))
    }
}

/// ARM64 assembly text emission — generates readable .s files
/// Supports full HEXA-IR: arithmetic, memory, control flow, function calls
pub mod arm64_asm {
    use crate::ir::*;

    /// Emit a complete program as ARM64 assembly text
    pub fn emit_program_asm(functions: &[HexaFunction]) -> String {
        let mut asm = String::new();
        asm.push_str(".section __TEXT,__text,regular,pure_instructions\n");
        asm.push_str(".align 4\n\n");

        // Emit all functions, marking main as global entry point
        for func in functions {
            let label = if func.name == "main" { "_main" } else { &format!("_{}", func.name) };
            if func.name == "main" {
                asm.push_str(&format!(".global {}\n", label));
            }
            emit_function(&mut asm, func, label);
            asm.push('\n');
        }

        asm
    }

    /// Emit a single function as ARM64 assembly
    fn emit_function(asm: &mut String, func: &HexaFunction, label: &str) {
        // Calculate stack frame: 1 slot per SSA register
        // Parameters are bound to SSA regs 0..n_params-1 by the lowering,
        // so all registers (params + locals) share one unified slot space.
        let max_reg = func.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .flat_map(|i| {
                let mut regs = i.args.clone();
                if let Some(d) = i.dest { regs.push(d); }
                regs.into_iter()
            })
            .max()
            .unwrap_or(0) + 1;
        let num_params = func.params.len();
        let total_slots = std::cmp::max(max_reg, num_params);
        // 16-byte aligned, minimum 32 for frame + lr
        let frame_size = ((total_slots * 8 + 16 + 15) / 16) * 16;
        let frame_size = std::cmp::max(frame_size, 32);

        let is_main = label == "_main";

        asm.push_str(&format!("{}:\n", label));

        // Prologue: save fp+lr, set up frame
        // ARM64 stp pre-indexed max offset = 504, so use sub sp for large frames
        if frame_size <= 504 {
            asm.push_str(&format!("    stp x29, x30, [sp, #-{}]!\n", frame_size));
        } else {
            asm.push_str(&format!("    sub sp, sp, #{}\n", frame_size));
            asm.push_str("    stp x29, x30, [sp]\n");
        }
        asm.push_str("    mov x29, sp\n");

        // Store incoming parameters from x0-x7 to their stack slots
        // Parameters are SSA regs 0..n_params-1, so they go to slot(0), slot(1), etc.
        for i in 0..num_params.min(8) {
            let offset = 16 + i * 8; // after saved fp+lr
            asm.push_str(&format!("    str x{}, [sp, #{}]\n", i, offset));
        }

        // Helper: stack offset for SSA register (unified: params and locals share space)
        let slot = |reg: usize| -> usize {
            16 + reg * 8
        };

        // Emit blocks
        for block in &func.blocks {
            asm.push_str(&format!(".L{}_b{}:\n", label, block.id));

            for instr in &block.instrs {
                match instr.op {
                    HexaOp::Alloc => {
                        if let Some(dest) = instr.dest {
                            let imm = instr.args.first().copied().unwrap_or(0);
                            if matches!(instr.ty, HexaType::I64 | HexaType::Bool | HexaType::Void) {
                                asm.push_str(&format!("    mov x9, #{}\n", imm));
                                asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                            } else {
                                // Other types: store 0 placeholder
                                asm.push_str(&format!("    str xzr, [sp, #{}]\n", slot(dest)));
                            }
                        }
                    }

                    HexaOp::Load => {
                        if let (Some(dest), Some(&src)) = (instr.dest, instr.args.first()) {
                            // Load from the slot of the source SSA register
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(src)));
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    HexaOp::Store => {
                        // Store: args[0] = addr_reg (destination slot), args[1] = value_reg
                        if let (Some(&addr), Some(&val)) = (instr.args.get(0), instr.args.get(1)) {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(val)));
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(addr)));
                        }
                    }

                    HexaOp::Add => {
                        if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(a)));
                            asm.push_str(&format!("    ldr x10, [sp, #{}]\n", slot(b)));
                            asm.push_str("    add x9, x9, x10\n");
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    HexaOp::Sub => {
                        if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(a)));
                            asm.push_str(&format!("    ldr x10, [sp, #{}]\n", slot(b)));
                            if matches!(instr.ty, HexaType::Bool) {
                                // Comparison: emit cmp + cset with correct condition
                                let cond = match instr.label.as_deref() {
                                    Some("eq") => "eq",
                                    Some("ne") => "ne",
                                    Some("lt") => "lt",
                                    Some("gt") => "gt",
                                    Some("le") => "le",
                                    Some("ge") => "ge",
                                    _ => "eq", // default fallback
                                };
                                asm.push_str("    cmp x9, x10\n");
                                asm.push_str(&format!("    cset x9, {}\n", cond));
                            } else {
                                asm.push_str("    sub x9, x9, x10\n");
                            }
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    HexaOp::Mul => {
                        if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(a)));
                            asm.push_str(&format!("    ldr x10, [sp, #{}]\n", slot(b)));
                            asm.push_str("    mul x9, x9, x10\n");
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    HexaOp::Div => {
                        if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(a)));
                            asm.push_str(&format!("    ldr x10, [sp, #{}]\n", slot(b)));
                            asm.push_str("    sdiv x9, x9, x10\n");
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    HexaOp::Mod => {
                        if let (Some(dest), Some(&a), Some(&b)) = (instr.dest, instr.args.get(0), instr.args.get(1)) {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(a)));
                            asm.push_str(&format!("    ldr x10, [sp, #{}]\n", slot(b)));
                            asm.push_str("    sdiv x11, x9, x10\n");
                            asm.push_str("    msub x9, x11, x10, x9\n");
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    HexaOp::Neg => {
                        if let (Some(dest), Some(&a)) = (instr.dest, instr.args.first()) {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(a)));
                            asm.push_str("    neg x9, x9\n");
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    HexaOp::Return => {
                        if let Some(&arg) = instr.args.first() {
                            asm.push_str(&format!("    ldr x0, [sp, #{}]\n", slot(arg)));
                        }

                        if is_main {
                            // For main: use exit syscall so process exits with return value
                            // mov x16, #1 (SYS_exit)
                            asm.push_str("    mov x16, #1\n");
                            asm.push_str("    svc #0x80\n");
                        } else {
                            // Epilogue: restore frame and return
                            if frame_size <= 504 {
                                asm.push_str(&format!("    ldp x29, x30, [sp], #{}\n", frame_size));
                            } else {
                                asm.push_str("    ldp x29, x30, [sp]\n");
                                asm.push_str(&format!("    add sp, sp, #{}\n", frame_size));
                            }
                            asm.push_str("    ret\n");
                        }
                    }

                    HexaOp::Branch => {
                        // args[0] = cond_reg, args[1] = then_block, args[2] = else_block
                        if let (Some(&cond), Some(&then_id), Some(&else_id)) =
                            (instr.args.get(0), instr.args.get(1), instr.args.get(2))
                        {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(cond)));
                            asm.push_str(&format!("    cbnz x9, .L{}_b{}\n", label, then_id));
                            asm.push_str(&format!("    b .L{}_b{}\n", label, else_id));
                        }
                    }

                    HexaOp::Jump => {
                        if let Some(&target_id) = instr.args.first() {
                            asm.push_str(&format!("    b .L{}_b{}\n", label, target_id));
                        }
                    }

                    HexaOp::Call => {
                        // args[0] = func_reg (Ident lowered), args[1..] = actual args
                        // Move arguments to x0-x7
                        if instr.args.len() > 1 {
                            for (i, &arg_reg) in instr.args[1..].iter().enumerate() {
                                if i < 8 {
                                    asm.push_str(&format!("    ldr x{}, [sp, #{}]\n", i, slot(arg_reg)));
                                }
                            }
                        }

                        // Use the label field to get the function name
                        let func_name = instr.label.as_deref().unwrap_or("unknown");
                        asm.push_str(&format!("    bl _{}\n", func_name));

                        // Store result (x0) to dest slot
                        if let Some(dest) = instr.dest {
                            asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    // Proof ops: zero runtime cost
                    HexaOp::ProofAssert | HexaOp::ProofInvariant | HexaOp::ProofWitness |
                    HexaOp::OwnershipTransfer | HexaOp::BorrowCheck | HexaOp::LifetimeEnd => {}

                    // Other ops: nop for Mk.I
                    HexaOp::Free | HexaOp::Copy | HexaOp::Move | HexaOp::Phi | HexaOp::Switch => {}
                }
            }
        }
    }

}

/// Generate native binary from optimized IR
pub fn generate(func: &HexaFunction, target: &Target) -> Vec<u8> {
    // Step 1: Register allocation
    let alloc = regalloc::allocate(func);

    // Step 2: Instruction selection
    let machine_code = match target {
        Target::X86_64Linux | Target::X86_64MacOS => {
            x86_64::select_function(func, &alloc)
        }
        Target::Arm64MacOS => {
            arm64::select_function(func, &alloc)
        }
    };

    // Step 3: Binary emission
    match target {
        Target::X86_64Linux => {
            elf::write_elf(&machine_code, 0x401000)
        }
        Target::X86_64MacOS => {
            macho::write_macho(&machine_code, 0x100000000, false)
        }
        Target::Arm64MacOS => {
            macho::write_macho(&machine_code, 0x100000000, true)
        }
    }
}
