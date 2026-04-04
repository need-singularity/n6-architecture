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

    /// ARM64 12-bit immediate limit for add/sub instructions
    const ARM64_IMM12_MAX: usize = 4095;

    /// Emit sub sp, sp, #imm handling large immediates via movz/movk + sub
    fn emit_sub_sp(asm: &mut String, imm: usize) {
        if imm == 0 { return; }
        if imm <= ARM64_IMM12_MAX {
            asm.push_str(&format!("    sub sp, sp, #{}\n", imm));
        } else {
            // Load large immediate into x16 (scratch), then sub sp, sp, x16
            emit_load_imm(asm, "x16", imm as u64);
            asm.push_str("    sub sp, sp, x16\n");
        }
    }

    /// Emit add sp, sp, #imm handling large immediates via movz/movk + add
    fn emit_add_sp(asm: &mut String, imm: usize) {
        if imm == 0 { return; }
        if imm <= ARM64_IMM12_MAX {
            asm.push_str(&format!("    add sp, sp, #{}\n", imm));
        } else {
            // Load large immediate into x16 (scratch), then add sp, sp, x16
            emit_load_imm(asm, "x16", imm as u64);
            asm.push_str("    add sp, sp, x16\n");
        }
    }

    /// Emit add rd, sp, #imm handling large immediates
    fn emit_add_reg_sp_imm(asm: &mut String, rd: &str, imm: usize) {
        if imm == 0 {
            asm.push_str(&format!("    mov {}, sp\n", rd));
        } else if imm <= ARM64_IMM12_MAX {
            asm.push_str(&format!("    add {}, sp, #{}\n", rd, imm));
        } else {
            // Load large immediate into rd, then add rd, sp, rd
            emit_load_imm(asm, rd, imm as u64);
            asm.push_str(&format!("    add {}, sp, {}\n", rd, rd));
        }
    }

    /// Emit movz/movk sequence to load a 64-bit immediate into a register
    fn emit_load_imm(asm: &mut String, reg: &str, imm: u64) {
        let lo16 = imm & 0xFFFF;
        asm.push_str(&format!("    movz {}, #{}\n", reg, lo16));
        let hi16 = (imm >> 16) & 0xFFFF;
        if hi16 != 0 {
            asm.push_str(&format!("    movk {}, #{}, lsl #16\n", reg, hi16));
        }
        let hi32 = (imm >> 32) & 0xFFFF;
        if hi32 != 0 {
            asm.push_str(&format!("    movk {}, #{}, lsl #32\n", reg, hi32));
        }
        let hi48 = (imm >> 48) & 0xFFFF;
        if hi48 != 0 {
            asm.push_str(&format!("    movk {}, #{}, lsl #48\n", reg, hi48));
        }
    }

    /// Emit a complete program as ARM64 assembly text
    pub fn emit_program_asm(functions: &[HexaFunction]) -> String {
        let mut asm = String::new();

        // Emit string constant pool in __cstring section
        let mut has_strings = false;
        for func in functions {
            if !func.string_pool.is_empty() {
                has_strings = true;
                break;
            }
        }
        if has_strings {
            asm.push_str(".section __TEXT,__cstring,cstring_literals\n");
            for func in functions {
                for (i, s) in func.string_pool.iter().enumerate() {
                    asm.push_str(&format!("_str_{}_{}:\n", func.name, i));
                    // Emit .asciz (null-terminated string literal)
                    asm.push_str(&format!("    .asciz \"{}\"\n",
                        s.replace('\\', "\\\\")
                         .replace('"', "\\\"")
                         .replace('\n', "\\n")
                         .replace('\t', "\\t")
                         .replace('\0', "\\0")));
                }
            }
            asm.push('\n');
        }

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

        // Pre-scan for aggregate allocations (arrays and structs):
        // compute extra stack space beyond the SSA register slots
        let mut array_allocs: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut array_data_size: usize = 0;
        for block in &func.blocks {
            for instr in &block.instrs {
                if instr.op == HexaOp::Alloc {
                    if let Some(dest) = instr.dest {
                        if matches!(instr.ty, HexaType::Array(_, _) | HexaType::Struct(_)) {
                            let size = instr.args.first().copied().unwrap_or(0);
                            array_allocs.insert(dest, array_data_size);
                            array_data_size += size;
                        }
                    }
                }
            }
        }

        // 16-byte aligned, minimum 32 for frame + lr
        let slots_bytes = total_slots * 8 + 16;
        let total_frame_bytes = slots_bytes + array_data_size;
        let frame_size = ((total_frame_bytes + 15) / 16) * 16;
        let frame_size = std::cmp::max(frame_size, 32);
        // Aggregate data starts after the SSA register slots
        let array_base_offset = 16 + total_slots * 8;

        let is_main = label == "_main";

        asm.push_str(&format!("{}:\n", label));

        // Prologue: save fp+lr, set up frame
        // ARM64 stp pre-indexed max offset = 504, so use sub sp for large frames
        if frame_size <= 504 {
            asm.push_str(&format!("    stp x29, x30, [sp, #-{}]!\n", frame_size));
        } else {
            // Large frame: use helper that handles >4095 via movz/movk + sub
            emit_sub_sp(asm, frame_size);
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

        // Track which SSA registers hold memory addresses (pointer-based Store/Load)
        // A register is a "pointer" if defined by Array/Struct Alloc or Add involving a pointer
        let mut pointer_regs: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for block in &func.blocks {
            for instr in &block.instrs {
                if let Some(dest) = instr.dest {
                    if instr.op == HexaOp::Alloc && array_allocs.contains_key(&dest) {
                        pointer_regs.insert(dest);
                    }
                    if instr.op == HexaOp::Add {
                        if instr.args.iter().any(|a| pointer_regs.contains(a)) {
                            pointer_regs.insert(dest);
                        }
                    }
                }
            }
        }

        // Emit blocks
        for block in &func.blocks {
            asm.push_str(&format!(".L{}_b{}:\n", label, block.id));

            for instr in &block.instrs {
                match instr.op {
                    HexaOp::Alloc => {
                        if let Some(dest) = instr.dest {
                            if let Some(&arr_offset) = array_allocs.get(&dest) {
                                // Array allocation: store the stack address of array data
                                // into this SSA register's slot
                                let addr = array_base_offset + arr_offset;
                                emit_add_reg_sp_imm(asm, "x9", addr);
                                asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                            } else if matches!(instr.ty, HexaType::Str) && instr.label.is_some() {
                                // String literal: load pointer to string constant from pool
                                let pool_idx = instr.args.first().copied().unwrap_or(0);
                                let str_len = instr.args.get(1).copied().unwrap_or(0);
                                let str_label = format!("_str_{}_{}", func.name, pool_idx);
                                asm.push_str(&format!("    adrp x9, {}@PAGE\n", str_label));
                                asm.push_str(&format!("    add x9, x9, {}@PAGEOFF\n", str_label));
                                asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                                let _ = str_len;
                            } else {
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
                    }

                    HexaOp::Load => {
                        if let (Some(dest), Some(&src)) = (instr.dest, instr.args.first()) {
                            if pointer_regs.contains(&src) {
                                // Pointer-based load: src holds a memory address
                                // Load the address, then load the value from that address
                                asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(src)));
                                asm.push_str("    ldr x9, [x9]\n");
                                asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                            } else {
                                // Slot-copy load: copy value between SSA register slots
                                asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(src)));
                                asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                            }
                        }
                    }

                    HexaOp::Store => {
                        // Store: args[0] = addr_reg, args[1] = value_reg
                        if let (Some(&addr), Some(&val)) = (instr.args.get(0), instr.args.get(1)) {
                            if pointer_regs.contains(&addr) {
                                // Pointer-based store: addr holds a memory address
                                // Load the address, load the value, then store value at address
                                asm.push_str(&format!("    ldr x10, [sp, #{}]\n", slot(addr)));
                                asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(val)));
                                asm.push_str("    str x9, [x10]\n");
                            } else {
                                // Slot-copy store: copy value to the destination slot
                                asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(val)));
                                asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(addr)));
                            }
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
                                // Large frame: use helper that handles >4095 via movz/movk + add
                                emit_add_sp(asm, frame_size);
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
                        let call_name = instr.label.as_deref().unwrap_or("unknown");

                        match call_name {
                            // ── Built-in: print(s: str) -> i64 ──
                            // write(stdout=1, str_ptr, str_len) via syscall x16=4
                            "print" => {
                                if instr.args.len() > 1 {
                                    let str_reg = instr.args[1];
                                    asm.push_str(&format!("    ldr x1, [sp, #{}]\n", slot(str_reg)));
                                    // Find string length from the Alloc that defined str_reg
                                    let str_len = func.blocks.iter()
                                        .flat_map(|b| b.instrs.iter())
                                        .find(|i| i.dest == Some(str_reg)
                                            && i.op == HexaOp::Alloc
                                            && matches!(i.ty, HexaType::Str))
                                        .and_then(|i| i.args.get(1).copied())
                                        .unwrap_or(0);
                                    asm.push_str(&format!("    mov x2, #{}\n", str_len));
                                    asm.push_str("    mov x0, #1\n");    // fd = stdout
                                    asm.push_str("    mov x16, #4\n");   // SYS_write
                                    asm.push_str("    svc #0x80\n");
                                }
                                if let Some(dest) = instr.dest {
                                    asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                                }
                            }

                            // ── Built-in: file_open(path: str) -> i64 (fd) ──
                            "file_open" => {
                                if instr.args.len() > 1 {
                                    asm.push_str(&format!("    ldr x0, [sp, #{}]\n", slot(instr.args[1])));
                                }
                                asm.push_str("    mov x1, #0\n");
                                asm.push_str("    mov x2, #0\n");
                                asm.push_str("    mov x16, #5\n");   // SYS_open
                                asm.push_str("    svc #0x80\n");
                                if let Some(dest) = instr.dest {
                                    asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                                }
                            }

                            // ── Built-in: file_read(fd, buf, n) -> i64 ──
                            "file_read" => {
                                if instr.args.len() > 3 {
                                    asm.push_str(&format!("    ldr x0, [sp, #{}]\n", slot(instr.args[1])));
                                    asm.push_str(&format!("    ldr x1, [sp, #{}]\n", slot(instr.args[2])));
                                    asm.push_str(&format!("    ldr x2, [sp, #{}]\n", slot(instr.args[3])));
                                }
                                asm.push_str("    mov x16, #3\n");   // SYS_read
                                asm.push_str("    svc #0x80\n");
                                if let Some(dest) = instr.dest {
                                    asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                                }
                            }

                            // ── Built-in: file_write(fd, data, n) -> i64 ──
                            "file_write" => {
                                if instr.args.len() > 3 {
                                    asm.push_str(&format!("    ldr x0, [sp, #{}]\n", slot(instr.args[1])));
                                    asm.push_str(&format!("    ldr x1, [sp, #{}]\n", slot(instr.args[2])));
                                    asm.push_str(&format!("    ldr x2, [sp, #{}]\n", slot(instr.args[3])));
                                }
                                asm.push_str("    mov x16, #4\n");   // SYS_write
                                asm.push_str("    svc #0x80\n");
                                if let Some(dest) = instr.dest {
                                    asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                                }
                            }

                            // ── Built-in: file_close(fd) -> i64 ──
                            "file_close" => {
                                if instr.args.len() > 1 {
                                    asm.push_str(&format!("    ldr x0, [sp, #{}]\n", slot(instr.args[1])));
                                }
                                asm.push_str("    mov x16, #6\n");   // SYS_close
                                asm.push_str("    svc #0x80\n");
                                if let Some(dest) = instr.dest {
                                    asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                                }
                            }

                            // ── Built-in: heap_alloc(size) -> i64 (pointer) ──
                            // mmap(0, size, PROT_READ|PROT_WRITE=3, MAP_ANON|MAP_PRIVATE=0x1002, -1, 0)
                            "heap_alloc" => {
                                if instr.args.len() > 1 {
                                    asm.push_str(&format!("    ldr x1, [sp, #{}]\n", slot(instr.args[1])));
                                }
                                asm.push_str("    mov x0, #0\n");
                                asm.push_str("    mov x2, #3\n");         // PROT_READ|PROT_WRITE
                                asm.push_str("    mov x3, #0x1002\n");    // MAP_ANON|MAP_PRIVATE
                                asm.push_str("    mov x4, #-1\n");
                                asm.push_str("    mov x5, #0\n");
                                asm.push_str("    mov x16, #197\n");      // SYS_mmap
                                asm.push_str("    svc #0x80\n");
                                if let Some(dest) = instr.dest {
                                    asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                                }
                            }

                            // ── Built-in: heap_free(ptr, size) -> i64 ──
                            // munmap(ptr, size) via syscall x16=73
                            "heap_free" => {
                                if instr.args.len() > 2 {
                                    asm.push_str(&format!("    ldr x0, [sp, #{}]\n", slot(instr.args[1])));
                                    asm.push_str(&format!("    ldr x1, [sp, #{}]\n", slot(instr.args[2])));
                                }
                                asm.push_str("    mov x16, #73\n");   // SYS_munmap
                                asm.push_str("    svc #0x80\n");
                                if let Some(dest) = instr.dest {
                                    asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                                }
                            }

                            // ── Regular function call ──
                            _ => {
                                if instr.args.len() > 1 {
                                    for (i, &arg_reg) in instr.args[1..].iter().enumerate() {
                                        if i < 8 {
                                            asm.push_str(&format!("    ldr x{}, [sp, #{}]\n", i, slot(arg_reg)));
                                        }
                                    }
                                }
                                asm.push_str(&format!("    bl _{}\n", call_name));
                                if let Some(dest) = instr.dest {
                                    asm.push_str(&format!("    str x0, [sp, #{}]\n", slot(dest)));
                                }
                            }
                        }
                    }

                    // Proof ops: zero runtime cost
                    HexaOp::ProofAssert | HexaOp::ProofInvariant | HexaOp::ProofWitness |
                    HexaOp::OwnershipTransfer | HexaOp::BorrowCheck | HexaOp::LifetimeEnd => {}

                    // Copy/Move: copy value from source slot to dest slot
                    HexaOp::Copy | HexaOp::Move => {
                        if let (Some(dest), Some(&src)) = (instr.dest, instr.args.first()) {
                            asm.push_str(&format!("    ldr x9, [sp, #{}]\n", slot(src)));
                            asm.push_str(&format!("    str x9, [sp, #{}]\n", slot(dest)));
                        }
                    }

                    // Other ops: nop for Mk.I
                    HexaOp::Free | HexaOp::Phi | HexaOp::Switch => {}
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
