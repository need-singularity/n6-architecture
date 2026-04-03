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
    let mut all_code = Vec::new();
    for func in functions {
        let alloc = regalloc::allocate(func);
        let mc = match &target {
            Target::X86_64Linux | Target::X86_64MacOS => x86_64::select_function(func, &alloc),
            Target::Arm64MacOS => arm64::select_function(func, &alloc),
        };
        all_code.extend_from_slice(&mc);
    }

    // Write assembly with raw bytes as .byte directives
    let asm_path = format!("{}.s", output);
    let mut asm = String::new();
    asm.push_str(".global _main\n_main:\n");
    for byte in &all_code {
        asm.push_str(&format!("    .byte 0x{:02x}\n", byte));
    }

    std::fs::write(&asm_path, &asm).map_err(|e| format!("write asm: {}", e))?;

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
