//! HEXA-IR Compiler Library — n=6 Native Compilation Stack
//!
//! σ(n)·φ(n) = n·τ(n) ⟺ n = 6
//! LLVM-independent compiler: J₂=24 opcodes, σ=12 passes, proof-preserving pipeline
//!
//! Module dependency (topological order):
//!   util → ir → diag → proof → alloc → lexer → parser → sema → lower → opt → codegen
#![allow(dead_code)]

// ═══ Foundation ═══
pub mod util;
pub mod ir;

// ═══ Support ═══
pub mod diag;
pub mod proof;
pub mod alloc;

// ═══ Pipeline (n=6 stages) ═══
pub mod lexer;    // Stage 1: Source → Tokens
pub mod parser;   // Stage 2: Tokens → AST
pub mod sema;     // Stage 3: AST → Typed AST
pub mod lower;    // Stage 4: AST → HEXA-IR
pub mod opt;      // Stage 5: IR → Optimized IR (σ=12 passes)
pub mod codegen;  // Stage 6: IR → Native Binary

#[cfg(test)]
mod string_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::HexaType;

    /// Helper: run full pipeline (lex → parse → sema → lower) and return IR functions
    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_string_literal_basic() {
        // Simple string literal assignment + return 0
        let source = r#"
fn main() -> i64 {
    let s = "hello";
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        assert_eq!(funcs.len(), 1);
        let main_fn = &funcs[0];
        assert_eq!(main_fn.name, "main");

        // String pool should contain "hello"
        assert_eq!(main_fn.string_pool.len(), 1);
        assert_eq!(main_fn.string_pool[0], "hello");
    }

    #[test]
    fn test_string_literal_escape_sequences() {
        let source = r#"
fn main() -> i64 {
    let s = "hello\nworld";
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        assert_eq!(main_fn.string_pool.len(), 1);
        assert_eq!(main_fn.string_pool[0], "hello\nworld");
    }

    #[test]
    fn test_string_pool_dedup() {
        // Same string used twice should be interned once
        let source = r#"
fn main() -> i64 {
    let a = "hello";
    let b = "hello";
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        assert_eq!(main_fn.string_pool.len(), 1);
        assert_eq!(main_fn.string_pool[0], "hello");
    }

    #[test]
    fn test_string_pool_multiple() {
        let source = r#"
fn main() -> i64 {
    let a = "hello";
    let b = "world";
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        assert_eq!(main_fn.string_pool.len(), 2);
        assert_eq!(main_fn.string_pool[0], "hello");
        assert_eq!(main_fn.string_pool[1], "world");
    }

    #[test]
    fn test_string_type_checking() {
        // StrLit should type-check as Str
        let source = r#"
fn main() -> i64 {
    let s = "test";
    return 0;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        // Should pass sema without errors
        sema::analyze(&program).expect("sema should accept string literal");
    }

    #[test]
    fn test_string_ir_alloc_has_pool_index() {
        let source = r#"
fn main() -> i64 {
    let s = "hello";
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        // Find the Alloc instruction with Str type
        let str_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| matches!(i.ty, HexaType::Str) && i.label.is_some());

        assert!(str_alloc.is_some(), "should have a Str-typed Alloc instruction");
        let instr = str_alloc.unwrap();
        // args[0] = pool index (0), args[1] = string length (5)
        assert_eq!(instr.args.len(), 2);
        assert_eq!(instr.args[0], 0); // pool index
        assert_eq!(instr.args[1], 5); // "hello".len()
        assert_eq!(instr.label.as_deref(), Some("str_pool_0"));
    }
}

#[cfg(test)]
mod struct_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::{HexaType, HexaOp};

    /// Helper: run full pipeline (lex -> parse -> sema -> lower) and return IR functions
    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_struct_basic_init() {
        let source = r#"
struct Point { x: i64, y: i64 }

fn main() -> i64 {
    let p = Point { x: 10, y: 20 };
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        assert_eq!(funcs.len(), 1);
        let main_fn = &funcs[0];
        assert_eq!(main_fn.name, "main");

        // Should have a Struct-typed Alloc instruction
        let struct_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| matches!(i.ty, HexaType::Struct(_)) && i.op == HexaOp::Alloc);
        assert!(struct_alloc.is_some(), "should have a Struct-typed Alloc instruction");
        let alloc_instr = struct_alloc.unwrap();
        // Total size should be 16 bytes (2 * i64 = 2 * 8)
        assert_eq!(alloc_instr.args[0], 16, "struct alloc size should be 16 bytes");
        assert_eq!(alloc_instr.label.as_deref(), Some("struct_Point"));
    }

    #[test]
    fn test_struct_field_access() {
        let source = r#"
struct Point { x: i64, y: i64 }

fn main() -> i64 {
    let p = Point { x: 10, y: 20 };
    return p.x + p.y;
}
"#;
        let funcs = compile_to_ir(source);
        assert_eq!(funcs.len(), 1);
        let main_fn = &funcs[0];

        // Should have field access Load instructions with field labels
        let field_loads: Vec<_> = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Load && i.label.is_some())
            .collect();

        // Should have at least 2 field loads (p.x and p.y)
        assert!(field_loads.len() >= 2,
            "should have at least 2 field Load instructions, found {}",
            field_loads.len());

        // First field load should be for "x" (offset 0, direct from base)
        let x_load = field_loads.iter().find(|i| i.label.as_deref() == Some("field_x"));
        assert!(x_load.is_some(), "should have a Load for field 'x'");

        // Second field load should be for "y"
        let y_load = field_loads.iter().find(|i| i.label.as_deref() == Some("field_y"));
        assert!(y_load.is_some(), "should have a Load for field 'y'");
    }

    #[test]
    fn test_struct_field_offset() {
        let source = r#"
struct Point { x: i64, y: i64 }

fn main() -> i64 {
    let p = Point { x: 10, y: 20 };
    return p.y;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        // For p.y (second field), there should be an Alloc with value 8 (byte offset)
        // followed by an Add (base + offset), then a Load
        let alloc_8 = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc
                && matches!(i.ty, HexaType::I64)
                && i.args.first() == Some(&8)
                && i.label.is_none());
        assert!(alloc_8.is_some(),
            "should have an Alloc with offset=8 for second field access");
    }

    #[test]
    fn test_struct_sema_field_type() {
        // Verify that sema correctly resolves struct field types
        let source = r#"
struct Point { x: i64, y: i64 }

fn main() -> i64 {
    let p = Point { x: 10, y: 20 };
    return p.x + p.y;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        // Should pass sema without errors (field access types resolve correctly)
        sema::analyze(&program).expect("sema should accept struct field access");
    }

    #[test]
    fn test_struct_three_fields() {
        let source = r#"
struct Vec3 { x: i64, y: i64, z: i64 }

fn main() -> i64 {
    let v = Vec3 { x: 1, y: 2, z: 3 };
    return v.z;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        // Struct alloc should be 24 bytes (3 * 8)
        let struct_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| matches!(i.ty, HexaType::Struct(_)) && i.op == HexaOp::Alloc);
        assert!(struct_alloc.is_some());
        assert_eq!(struct_alloc.unwrap().args[0], 24, "Vec3 should be 24 bytes");

        // For v.z, offset should be 16 (2 * 8 bytes)
        let alloc_16 = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc
                && matches!(i.ty, HexaType::I64)
                && i.args.first() == Some(&16)
                && i.label.is_none());
        assert!(alloc_16.is_some(),
            "should have offset=16 for third field access");
    }

    #[test]
    fn test_struct_codegen_asm() {
        // Verify that ARM64 assembly generation handles structs without panicking
        let source = r#"
struct Point { x: i64, y: i64 }

fn main() -> i64 {
    let p = Point { x: 10, y: 20 };
    return p.x + p.y;
}
"#;
        let funcs = compile_to_ir(source);
        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);
        assert!(asm.contains("_main:"), "ASM should contain _main label");
        // Should contain struct data allocation (add x9, sp, #offset)
        assert!(asm.contains("add x9, sp,"), "ASM should allocate struct on stack");
    }
}

#[cfg(test)]
mod enum_match_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::{HexaType, HexaOp};

    /// Helper: run full pipeline (lex -> parse -> sema -> lower) and return IR functions
    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_enum_parse_and_lower() {
        let source = r#"
enum Color { Red, Green, Blue }

fn main() -> i64 {
    let c = Color::Green;
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        assert_eq!(funcs.len(), 1);
        let main_fn = &funcs[0];
        assert_eq!(main_fn.name, "main");

        // Color::Green should lower to an Alloc with tag=1 (second variant)
        let tag_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc
                && i.label.as_deref() == Some("enum_tag_Color::Green"));
        assert!(tag_alloc.is_some(), "should have an enum tag Alloc for Color::Green");
        assert_eq!(tag_alloc.unwrap().args[0], 1, "Green should have tag=1");
    }

    #[test]
    fn test_enum_variant_tags() {
        let source = r#"
enum Color { Red, Green, Blue }

fn main() -> i64 {
    let r = Color::Red;
    let g = Color::Green;
    let b = Color::Blue;
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        let tags: Vec<_> = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Alloc
                && i.label.as_ref().map_or(false, |l| l.starts_with("enum_tag_")))
            .collect();

        assert_eq!(tags.len(), 3, "should have 3 enum tag Allocs");
        assert_eq!(tags[0].args[0], 0, "Red tag = 0");
        assert_eq!(tags[1].args[0], 1, "Green tag = 1");
        assert_eq!(tags[2].args[0], 2, "Blue tag = 2");
    }

    #[test]
    fn test_match_parse() {
        // Verify that match expression parses and sema-checks correctly
        let source = r#"
enum Color { Red, Green, Blue }

fn color_val(c: Color) -> i64 {
    return match c {
        Color::Red => 1,
        Color::Green => 2,
        Color::Blue => 3,
    };
}

fn main() -> i64 {
    let c = Color::Green;
    return color_val(c);
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept match expression");
    }

    #[test]
    fn test_match_lowering_creates_blocks() {
        let source = r#"
enum Color { Red, Green, Blue }

fn color_val(c: Color) -> i64 {
    return match c {
        Color::Red => 1,
        Color::Green => 2,
        Color::Blue => 3,
    };
}

fn main() -> i64 {
    return 0;
}
"#;
        let funcs = compile_to_ir(source);

        // Find color_val function
        let color_fn = funcs.iter().find(|f| f.name == "color_val")
            .expect("should have color_val function");

        // Match should create multiple blocks (entry + 3 arm blocks + check blocks + merge)
        assert!(color_fn.blocks.len() >= 4,
            "match should create multiple blocks, found {}",
            color_fn.blocks.len());

        // Should have Branch instructions (compare-and-branch for each arm)
        let branch_count = color_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Branch)
            .count();
        assert!(branch_count >= 2,
            "should have at least 2 Branch instructions for 3-arm match, found {}",
            branch_count);

        // Should have Jump instructions (arm -> merge)
        let jump_count = color_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Jump)
            .count();
        assert!(jump_count >= 3,
            "should have at least 3 Jump instructions (one per arm), found {}",
            jump_count);
    }

    #[test]
    fn test_match_with_wildcard() {
        let source = r#"
enum Color { Red, Green, Blue }

fn is_red(c: Color) -> i64 {
    return match c {
        Color::Red => 1,
        _ => 0,
    };
}

fn main() -> i64 {
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let is_red_fn = funcs.iter().find(|f| f.name == "is_red")
            .expect("should have is_red function");

        // Should compile without errors and have multiple blocks
        assert!(is_red_fn.blocks.len() >= 3,
            "wildcard match should create multiple blocks, found {}",
            is_red_fn.blocks.len());
    }

    #[test]
    fn test_enum_sema_type_check() {
        // Verify enum type names are registered in sema
        let source = r#"
enum Direction { Up, Down, Left, Right }

fn main() -> i64 {
    let d = Direction::Up;
    return 0;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept enum usage");
    }

    // ── Array tests ──

    #[test]
    fn test_array_literal_parse_and_sema() {
        let source = r#"
fn main() -> i64 {
    let arr: [i64; 3] = [10, 20, 30]
    return arr[1]
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept array literal");
    }

    #[test]
    fn test_array_ir_alloc_type() {
        let source = r#"
fn main() -> i64 {
    let arr: [i64; 3] = [10, 20, 30]
    return arr[0]
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        // First instruction should be Array allocation
        let alloc = &main_fn.blocks[0].instrs[0];
        assert_eq!(alloc.op, crate::ir::HexaOp::Alloc);
        assert!(matches!(alloc.ty, HexaType::Array(_, 3)),
            "Array alloc should have Array(I64, 3) type, got {:?}", alloc.ty);
        // Total allocation = 3 * 8 = 24 bytes
        assert_eq!(alloc.args[0], 24, "Array alloc should request 24 bytes");
    }

    #[test]
    fn test_array_ir_stores_elements() {
        let source = r#"
fn main() -> i64 {
    let arr: [i64; 3] = [10, 20, 30]
    return arr[0]
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        // Count Store instructions: should be 3 (one per element)
        let store_count = main_fn.blocks[0].instrs.iter()
            .filter(|i| i.op == crate::ir::HexaOp::Store)
            .count();
        assert_eq!(store_count, 3, "Should have 3 Store instructions for 3 array elements");
    }

    #[test]
    fn test_array_index_has_proof_assert() {
        let source = r#"
fn main() -> i64 {
    let arr: [i64; 3] = [10, 20, 30]
    return arr[1]
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        // Should have a ProofAssert for bounds checking
        let has_proof = main_fn.blocks[0].instrs.iter()
            .any(|i| i.op == crate::ir::HexaOp::ProofAssert);
        assert!(has_proof, "Array index should emit ProofAssert for bounds checking");
    }

    #[test]
    fn test_array_index_element_size_scaling() {
        let source = r#"
fn main() -> i64 {
    let arr: [i64; 3] = [10, 20, 30]
    return arr[2]
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        // Should have a Mul instruction for index * element_size
        let has_mul = main_fn.blocks[0].instrs.iter()
            .any(|i| i.op == crate::ir::HexaOp::Mul);
        assert!(has_mul, "Array index should emit Mul for element size scaling");
    }

    #[test]
    fn test_array_sema_element_type() {
        // Verify sema infers correct element type from array
        let source = r#"
fn main() -> i64 {
    let arr: [i64; 3] = [10, 20, 30]
    return arr[0]
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        // Should pass sema: arr[0] has type i64, matches return type
        sema::analyze(&program).expect("sema should accept array index as i64");
    }

    #[test]
    fn test_array_codegen_asm() {
        // Verify ARM64 assembly generation handles arrays without panicking
        let source = r#"
fn main() -> i64 {
    let arr: [i64; 3] = [10, 20, 30]
    return arr[1]
}
"#;
        let mut funcs = compile_to_ir(source);
        for func in &mut funcs {
            crate::opt::run_pipeline(func);
        }
        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);
        // Assembly should contain array base address computation
        assert!(asm.contains("add x9, sp, #"), "Should compute array base from sp");
        // Assembly should contain pointer-based stores
        assert!(asm.contains("str x9, [x10]") || asm.contains("str x9, [x"),
            "Should have pointer-based stores for array elements");
    }
}

#[cfg(test)]
mod builtin_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::{HexaType, HexaOp};

    /// Helper: full pipeline to IR
    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_print_sema_accepts() {
        let source = r#"
fn main() -> i64 {
    print("hello world\n");
    return 0;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept print() built-in");
    }

    #[test]
    fn test_print_lowers_to_call() {
        let source = r#"
fn main() -> i64 {
    print("hello\n");
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        let print_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Call && i.label.as_deref() == Some("print"));
        assert!(print_call.is_some(), "should have a Call to 'print'");

        assert_eq!(main_fn.string_pool.len(), 1);
        assert_eq!(main_fn.string_pool[0], "hello\n");
    }

    #[test]
    fn test_print_codegen_emits_syscall() {
        let source = r#"
fn main() -> i64 {
    print("hello world\n");
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);

        assert!(asm.contains("_str_main_0:"), "ASM should have string constant label");
        assert!(asm.contains("hello world"), "ASM should contain the string literal");
        assert!(asm.contains("mov x16, #4"), "ASM should have SYS_write syscall number");
        assert!(asm.contains("svc #0x80"), "ASM should have svc instruction");
        assert!(asm.contains("mov x0, #1"), "ASM should set fd=1 (stdout)");
        assert!(!asm.contains("bl _print"), "print should be inlined, not a bl call");
    }

    #[test]
    fn test_file_io_sema_accepts() {
        let source = r#"
fn main() -> i64 {
    let fd = file_open("test.txt");
    let n = file_close(fd);
    return 0;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept file I/O built-ins");
    }

    #[test]
    fn test_file_io_lowers_to_calls() {
        let source = r#"
fn main() -> i64 {
    let fd = file_open("test.txt");
    let n = file_close(fd);
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        let open_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Call && i.label.as_deref() == Some("file_open"));
        assert!(open_call.is_some(), "should have Call to 'file_open'");

        let close_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Call && i.label.as_deref() == Some("file_close"));
        assert!(close_call.is_some(), "should have Call to 'file_close'");
    }

    #[test]
    fn test_file_open_codegen_emits_syscall() {
        let source = r#"
fn main() -> i64 {
    let fd = file_open("test.txt");
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);

        assert!(asm.contains("mov x16, #5"), "ASM should have SYS_open syscall number");
        assert!(!asm.contains("bl _file_open"), "file_open should be inlined");
    }

    #[test]
    fn test_heap_alloc_sema_accepts() {
        let source = r#"
fn main() -> i64 {
    let ptr = heap_alloc(4096);
    let r = heap_free(ptr, 4096);
    return 0;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept heap built-ins");
    }

    #[test]
    fn test_heap_alloc_codegen_emits_mmap() {
        let source = r#"
fn main() -> i64 {
    let ptr = heap_alloc(4096);
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);

        assert!(asm.contains("mov x16, #197"), "ASM should have SYS_mmap syscall number");
        assert!(asm.contains("mov x3, #0x1002"), "ASM should have MAP_ANON|MAP_PRIVATE flags");
        assert!(!asm.contains("bl _heap_alloc"), "heap_alloc should be inlined");
    }

    #[test]
    fn test_heap_free_codegen_emits_munmap() {
        let source = r#"
fn main() -> i64 {
    let ptr = heap_alloc(4096);
    let r = heap_free(ptr, 4096);
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);

        assert!(asm.contains("mov x16, #73"), "ASM should have SYS_munmap syscall number");
    }

    #[test]
    fn test_all_builtins_coexist() {
        let source = r#"
fn main() -> i64 {
    print("start\n");
    let fd = file_open("data.txt");
    let n = file_close(fd);
    let ptr = heap_alloc(1024);
    let r = heap_free(ptr, 1024);
    print("done\n");
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        assert_eq!(funcs.len(), 1);
        let main_fn = &funcs[0];

        let call_count = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Call)
            .count();
        assert_eq!(call_count, 6, "should have 6 built-in calls");

        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);
        assert!(!asm.contains("bl _print"), "print should be inlined");
        assert!(!asm.contains("bl _file_open"), "file_open should be inlined");
        assert!(!asm.contains("bl _file_close"), "file_close should be inlined");
        assert!(!asm.contains("bl _heap_alloc"), "heap_alloc should be inlined");
        assert!(!asm.contains("bl _heap_free"), "heap_free should be inlined");

        assert!(asm.contains("mov x16, #4"), "should have SYS_write");
        assert!(asm.contains("mov x16, #5"), "should have SYS_open");
        assert!(asm.contains("mov x16, #6"), "should have SYS_close");
        assert!(asm.contains("mov x16, #197"), "should have SYS_mmap");
        assert!(asm.contains("mov x16, #73"), "should have SYS_munmap");
    }
}

#[cfg(test)]
mod module_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::HexaOp;

    /// Helper: run full pipeline (lex -> parse -> sema -> lower) and return IR functions
    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_module_define_and_call() {
        let source = r#"
mod math {
    pub fn add(a: i64, b: i64) -> i64 {
        return a + b
    }
}

fn main() -> i64 {
    return math::add(1, 2)
}
"#;
        let funcs = compile_to_ir(source);
        assert!(funcs.len() >= 2, "expected at least 2 functions, got {}", funcs.len());

        let math_add = funcs.iter().find(|f| f.name == "math__add");
        assert!(math_add.is_some(), "should have function 'math__add'");

        let main_fn = funcs.iter().find(|f| f.name == "main");
        assert!(main_fn.is_some(), "should have function 'main'");

        let has_call = main_fn.unwrap().blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Call && i.label.as_deref() == Some("math__add"));
        assert!(has_call, "main should call 'math__add'");
    }

    #[test]
    fn test_use_import_name_resolution() {
        let source = r#"
mod math {
    pub fn multiply(a: i64, b: i64) -> i64 {
        return a * b
    }
}

use math::multiply;

fn main() -> i64 {
    return multiply(3, 4)
}
"#;
        let funcs = compile_to_ir(source);
        let math_mul = funcs.iter().find(|f| f.name == "math__multiply");
        assert!(math_mul.is_some(), "should have 'math__multiply' function");

        let main_fn = funcs.iter().find(|f| f.name == "main");
        assert!(main_fn.is_some(), "should have 'main' function");
    }

    #[test]
    fn test_pub_private_visibility() {
        let source = r#"
mod secret {
    fn hidden(x: i64) -> i64 {
        return x
    }
}

fn main() -> i64 {
    return secret::hidden(42)
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        let result = sema::analyze(&program);
        assert!(result.is_err(), "sema should reject access to private module function");
    }

    #[test]
    fn test_module_sema_accepts_pub() {
        let source = r#"
mod utils {
    pub fn double(x: i64) -> i64 {
        return x * 2
    }
}

fn main() -> i64 {
    return utils::double(21)
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept pub module function access");
    }

    #[test]
    fn test_module_with_use_sema() {
        let source = r#"
mod io {
    pub fn read_val() -> i64 {
        return 42
    }
}

use io::read_val;

fn main() -> i64 {
    return read_val()
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept use-imported function");
    }
}

// ═══ Closure Tests ═══

#[cfg(test)]
mod closure_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::HexaOp;

    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_closure_parse_simple() {
        let source = "fn main() -> i64 {\n    let f = |x: i64| x + 1\n    return 0\n}\n";
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept closure");
    }

    #[test]
    fn test_closure_parse_multi_param() {
        let source = "fn main() -> i64 {\n    let add = |x: i64, y: i64| x + y\n    return 0\n}\n";
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept multi-param closure");
    }

    #[test]
    fn test_closure_parse_no_params() {
        let source = "fn main() -> i64 {\n    let f = || 42\n    return 0\n}\n";
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept zero-param closure");
    }

    #[test]
    fn test_closure_typecheck_infers_fn() {
        let source = "fn main() -> i64 {\n    let f = |x: i64| x + 1\n    return 0\n}\n";
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should infer Fn type for closure");
    }

    #[test]
    fn test_closure_lowering_creates_env() {
        let source = "fn main() -> i64 {\n    let x = 10\n    let f = |y: i64| x + y\n    return 0\n}\n";
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        let env_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Alloc && i.label.as_deref() == Some("closure_env"));
        assert!(env_alloc, "closure should allocate an environment struct");
    }

    #[test]
    fn test_closure_capture_stores() {
        let source = "fn main() -> i64 {\n    let a = 5\n    let b = 10\n    let f = |x: i64| a + b + x\n    return 0\n}\n";
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];
        let store_count = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Store)
            .count();
        assert!(store_count >= 3, "should have stores for inits + captures, found {}", store_count);
    }
}

// ═══ Generic Tests ═══

#[cfg(test)]
mod generic_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::HexaOp;

    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_generic_fn_parse() {
        let source = "fn identity<T>(x: T) -> T {\n    return x\n}\nfn main() -> i64 { return identity(42) }\n";
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        let id_fn = program.decls.iter().find_map(|d| {
            if let crate::parser::ast::Decl::FnDecl(f) = d {
                if f.name == "identity" { Some(f) } else { None }
            } else { None }
        });
        assert!(id_fn.is_some(), "should have identity function");
        assert_eq!(id_fn.unwrap().type_params, vec!["T".to_string()]);
    }

    #[test]
    fn test_generic_fn_sema() {
        let source = "fn identity<T>(x: T) -> T {\n    return x\n}\nfn main() -> i64 { return identity(42) }\n";
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept generic function");
    }

    #[test]
    fn test_generic_fn_multiple_params() {
        let source = "fn pair<A, B>(a: A, b: B) -> A {\n    return a\n}\nfn main() -> i64 { return pair(1, 2) }\n";
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept multi-param generic");
        let pair_fn = program.decls.iter().find_map(|d| {
            if let crate::parser::ast::Decl::FnDecl(f) = d {
                if f.name == "pair" { Some(f) } else { None }
            } else { None }
        }).unwrap();
        assert_eq!(pair_fn.type_params, vec!["A".to_string(), "B".to_string()]);
    }

    #[test]
    fn test_generic_fn_monomorphize() {
        let source = "fn identity<T>(x: T) -> T {\n    return x\n}\nfn main() -> i64 { return identity(42) }\n";
        let funcs = compile_to_ir(source);
        assert!(funcs.len() >= 2, "should have main + identity, found {}", funcs.len());
        let id_fn = funcs.iter().find(|f| f.name == "identity");
        assert!(id_fn.is_some(), "should have monomorphized identity function");
    }

    #[test]
    fn test_generic_fn_body_preserved() {
        let source = "fn add_n<T>(x: T, n: i64) -> T {\n    return x + n\n}\nfn main() -> i64 { return add_n(10, 5) }\n";
        let funcs = compile_to_ir(source);
        let add_fn = funcs.iter().find(|f| f.name == "add_n").expect("should have add_n");
        let has_add = add_fn.blocks.iter().flat_map(|b| b.instrs.iter()).any(|i| i.op == HexaOp::Add);
        assert!(has_add, "add_n should have an Add instruction");
    }

    #[test]
    fn test_generic_and_closure_together() {
        let source = "fn identity<T>(x: T) -> T {\n    return x\n}\nfn main() -> i64 {\n    let f = |x: i64| x * 2\n    return identity(42)\n}\n";
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept generic + closure");
        let funcs = compile_to_ir(source);
        assert!(funcs.len() >= 2, "should have main + identity");
    }
}

#[cfg(test)]
mod advanced_pattern_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::{HexaType, HexaOp};

    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_struct_destructure_pattern() {
        let source = r#"
struct Point { x: i64, y: i64 }

fn sum_point(p: Point) -> i64 {
    return match p {
        Point { x, y } => x + y,
    };
}

fn main() -> i64 {
    let p = Point { x: 10, y: 20 };
    return sum_point(p);
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept struct destructure pattern");

        let funcs = compile_to_ir(source);
        let sum_fn = funcs.iter().find(|f| f.name == "sum_point")
            .expect("should have sum_point function");

        let destr_loads: Vec<_> = sum_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Load
                && i.label.as_ref().map_or(false, |l| l.starts_with("destr_")))
            .collect();

        assert!(destr_loads.len() >= 2,
            "should have at least 2 destructure Load instructions, found {}",
            destr_loads.len());
    }

    #[test]
    fn test_guard_condition_pattern() {
        let source = r#"
fn classify(x: i64) -> i64 {
    return match x {
        n if n > 0 => 1,
        n if n < 0 => 2,
        _ => 0,
    };
}

fn main() -> i64 {
    return classify(42);
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept guard condition pattern");

        let funcs = compile_to_ir(source);
        let classify_ir = funcs.iter().find(|f| f.name == "classify")
            .expect("should have classify function in IR");

        assert!(classify_ir.blocks.len() >= 3,
            "guard match should create multiple blocks, found {}",
            classify_ir.blocks.len());
    }

    #[test]
    fn test_result_type_and_try_operator() {
        let source = r#"
enum Result { Ok(i64), Err(i64) }

fn might_fail(x: i64) -> Result {
    if x > 0 {
        return Result::Ok(x);
    }
    return Result::Err(0);
}

fn use_result(x: i64) -> Result {
    let val = might_fail(x)?;
    return Result::Ok(val + 1);
}

fn main() -> i64 {
    return 0;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept Result + ? operator");

        let funcs = compile_to_ir(source);
        let use_result_fn = funcs.iter().find(|f| f.name == "use_result")
            .expect("should have use_result function");

        let branch_count = use_result_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Branch)
            .count();
        assert!(branch_count >= 1,
            "try(?) should create at least 1 Branch, found {}", branch_count);

        let err_return = use_result_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Return
                && i.label.as_deref() == Some("try_err_propagate"));
        assert!(err_return.is_some(), "try(?) should emit early Return for Err propagation");
    }

    #[test]
    fn test_error_propagation_chain() {
        let source = r#"
enum Result { Ok(i64), Err(i64) }

fn step1(x: i64) -> Result {
    return Result::Ok(x + 1);
}

fn step2(x: i64) -> Result {
    return Result::Ok(x + 2);
}

fn chain(x: i64) -> Result {
    let a = step1(x)?;
    let b = step2(a)?;
    return Result::Ok(b);
}

fn main() -> i64 {
    return 0;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept error propagation chain");

        let funcs = compile_to_ir(source);
        let chain_fn = funcs.iter().find(|f| f.name == "chain")
            .expect("should have chain function");

        let branch_count = chain_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Branch)
            .count();
        assert!(branch_count >= 2,
            "two ? should create at least 2 Branches, found {}", branch_count);

        let err_returns = chain_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Return
                && i.label.as_deref() == Some("try_err_propagate"))
            .count();
        assert!(err_returns >= 2,
            "two ? should emit at least 2 early Returns, found {}", err_returns);

        assert!(chain_fn.blocks.len() >= 5,
            "chain with 2x? should create at least 5 blocks, found {}",
            chain_fn.blocks.len());
    }

    #[test]
    fn test_variable_binding_pattern() {
        let source = r#"
fn identity(x: i64) -> i64 {
    return match x {
        val => val + 1,
    };
}

fn main() -> i64 {
    return identity(5);
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept variable binding pattern");

        let funcs = compile_to_ir(source);
        assert!(funcs.len() >= 2, "should have identity and main functions");
    }

    #[test]
    fn test_nested_variant_pattern() {
        let source = r#"
enum Option { Some(i64), None }

fn unwrap_or(opt: Option) -> i64 {
    return match opt {
        Option::Some(v) => v,
        Option::None => 0,
    };
}

fn main() -> i64 {
    let x = Option::Some(42);
    return unwrap_or(x);
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept nested variant pattern");

        let funcs = compile_to_ir(source);
        let unwrap_fn = funcs.iter().find(|f| f.name == "unwrap_or")
            .expect("should have unwrap_or function");

        let variant_loads: Vec<_> = unwrap_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Load
                && i.label.as_ref().map_or(false, |l| l.starts_with("variant_field_")))
            .collect();

        assert!(variant_loads.len() >= 1,
            "should have at least 1 variant_field Load for binding 'v', found {}",
            variant_loads.len());
    }
}

#[cfg(test)]
mod trait_impl_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::HexaOp;

    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    #[test]
    fn test_trait_def_parse() {
        let source = r#"
trait Printable {
    fn describe(&self) -> i64;
}

struct Point { x: i64, y: i64 }

impl Printable for Point {
    fn describe(&self) -> i64 {
        return self.x;
    }
}

fn main() -> i64 {
    return 0;
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept trait + impl");
    }

    #[test]
    fn test_impl_block_typecheck() {
        // Good: all methods implemented
        let source_good = r#"
trait Greetable {
    fn greet(&self) -> i64;
}

struct Dog { age: i64 }

impl Greetable for Dog {
    fn greet(&self) -> i64 {
        return self.age;
    }
}

fn main() -> i64 { return 0; }
"#;
        let tokens = lexer::lex(source_good).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept complete impl");

        // Bad: missing method
        let source_bad = r#"
trait Greetable {
    fn greet(&self) -> i64;
    fn wave(&self) -> i64;
}

struct Cat { age: i64 }

impl Greetable for Cat {
    fn greet(&self) -> i64 {
        return self.age;
    }
}

fn main() -> i64 { return 0; }
"#;
        let tokens2 = lexer::lex(source_bad).expect("lex failed");
        let program2 = parser::parse(tokens2).expect("parse failed");
        let result = sema::analyze(&program2);
        assert!(result.is_err(), "sema should reject impl missing 'wave' method");
    }

    #[test]
    fn test_method_call_lowering() {
        let source = r#"
trait Measurable {
    fn area(&self) -> i64;
}

struct Rect { w: i64, h: i64 }

impl Measurable for Rect {
    fn area(&self) -> i64 {
        return self.w;
    }
}

fn main() -> i64 {
    let r = Rect { w: 6, h: 12 };
    let a = r.area();
    return a;
}
"#;
        let funcs = compile_to_ir(source);

        // Should have main + Rect__area
        assert!(funcs.len() >= 2, "should have at least 2 functions (main + Rect__area), found {}", funcs.len());

        let area_fn = funcs.iter().find(|f| f.name == "Rect__area");
        assert!(area_fn.is_some(), "should have mangled function 'Rect__area'");

        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main function");
        let method_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Call && i.label.as_deref() == Some("Rect__area"));
        assert!(method_call.is_some(), "main should have a Call to 'Rect__area'");
    }

    #[test]
    fn test_trait_multiple_methods() {
        let source = r#"
trait Shape {
    fn area(&self) -> i64;
    fn perimeter(&self) -> i64;
}

struct Square { side: i64 }

impl Shape for Square {
    fn area(&self) -> i64 {
        return self.side;
    }
    fn perimeter(&self) -> i64 {
        return self.side;
    }
}

fn main() -> i64 {
    let s = Square { side: 6 };
    let a = s.area();
    let p = s.perimeter();
    return a;
}
"#;
        let funcs = compile_to_ir(source);

        let area_fn = funcs.iter().find(|f| f.name == "Square__area");
        assert!(area_fn.is_some(), "should have 'Square__area'");
        let perim_fn = funcs.iter().find(|f| f.name == "Square__perimeter");
        assert!(perim_fn.is_some(), "should have 'Square__perimeter'");

        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");
        let area_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Call && i.label.as_deref() == Some("Square__area"));
        let perim_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Call && i.label.as_deref() == Some("Square__perimeter"));
        assert!(area_call, "main should call Square__area");
        assert!(perim_call, "main should call Square__perimeter");
    }
}

#[cfg(test)]
mod for_loop_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::HexaOp;
    use crate::parser::ast::{Stmt, Expr, BinOp};

    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    /// Test 1: Parse `for i in 0..10 { }` — range-based for loop
    #[test]
    fn test_for_range_parse() {
        let source = r#"
fn main() -> i64 {
    let mut sum = 0
    for i in 0..10 {
        sum = sum + i
    }
    return sum
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");

        let main_fn = match &program.decls[0] {
            crate::parser::ast::Decl::FnDecl(f) => f,
            _ => panic!("expected FnDecl"),
        };

        // Find ForLoop statement
        let has_for = main_fn.body.stmts.iter().any(|s| {
            if let Stmt::ForLoop { var, iterable, .. } = s {
                var == "i" && matches!(iterable, Expr::Binary { op: BinOp::Range, .. })
            } else {
                false
            }
        });
        assert!(has_for, "should have ForLoop with range iterable");
    }

    /// Test 2: Parse `for item in arr { }` — array iteration
    #[test]
    fn test_for_array_parse() {
        let source = r#"
fn main() -> i64 {
    let arr = [10, 20, 30]
    let mut total = 0
    for item in arr {
        total = total + item
    }
    return total
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");

        let main_fn = match &program.decls[0] {
            crate::parser::ast::Decl::FnDecl(f) => f,
            _ => panic!("expected FnDecl"),
        };

        let has_for = main_fn.body.stmts.iter().any(|s| {
            if let Stmt::ForLoop { var, iterable, .. } = s {
                var == "item" && matches!(iterable, Expr::Ident(name, _) if name == "arr")
            } else {
                false
            }
        });
        assert!(has_for, "should have ForLoop with array variable");
    }

    /// Test 3: For loop lowering creates header + body + exit blocks (desugared to while)
    #[test]
    fn test_for_range_lowering_creates_blocks() {
        let source = r#"
fn main() -> i64 {
    let mut sum = 0
    for i in 0..6 {
        sum = sum + i
    }
    return sum
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");

        // For-loop desugars to while: should have 4+ blocks
        // (entry, header, body, exit)
        assert!(main_fn.blocks.len() >= 4,
            "for-range should desugar to 4+ blocks (entry+header+body+exit), got {}",
            main_fn.blocks.len());

        // Should have a Branch instruction (from header)
        let has_branch = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Branch);
        assert!(has_branch, "desugared for should have Branch (from loop header)");

        // Should have comparison with "lt" label (Sub + Bool + "lt")
        let has_lt_cmp = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Sub && i.label.as_deref() == Some("lt"));
        assert!(has_lt_cmp, "desugared for 0..6 should have Sub with 'lt' label for comparison");

        // Should have Add instruction (for counter increment)
        let has_add = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Add);
        assert!(has_add, "desugared for should have Add for counter increment");
    }

    /// Test 4: For loop inclusive range and sema type checking
    #[test]
    fn test_for_inclusive_range_sema_and_lower() {
        let source = r#"
fn main() -> i64 {
    let mut product = 1
    for i in 1..=6 {
        product = product * i
    }
    return product
}
"#;
        // Sema should accept this (loop var 'i' is i64 inferred from range)
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept for-in with inclusive range");

        let funcs = compile_to_ir(source);
        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");

        // Inclusive range uses "le" label
        let has_le_cmp = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Sub && i.label.as_deref() == Some("le"));
        assert!(has_le_cmp, "desugared for 1..=6 should have Sub with 'le' label for <= comparison");
    }

    /// Test 5: Array for-loop lowering produces array_len + array_element loads
    #[test]
    fn test_for_array_lowering() {
        let source = r#"
fn main() -> i64 {
    let data = [1, 2, 3, 4, 5, 6]
    let mut sum = 0
    for x in data {
        sum = sum + x
    }
    return sum
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");

        // Should have "array_len" Copy instruction
        let has_array_len = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Copy && i.label.as_deref() == Some("array_len"));
        assert!(has_array_len, "array for-loop should emit Copy with 'array_len' label");

        // Should have "array_element" Load instruction
        let has_array_elem = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Load && i.label.as_deref() == Some("array_element"));
        assert!(has_array_elem, "array for-loop should emit Load with 'array_element' label");

        // Should have 4+ blocks like range for
        assert!(main_fn.blocks.len() >= 4,
            "array for-loop should desugar to 4+ blocks, got {}",
            main_fn.blocks.len());
    }

    /// Test 6: DotDotEq lexer token
    #[test]
    fn test_dotdoteq_token() {
        let source = "fn main() -> i64 { for i in 0..=5 { } return 0 }";
        let tokens = lexer::lex(source).expect("lex failed");
        let has_dotdoteq = tokens.iter().any(|t| t.kind == crate::lexer::TokenKind::DotDotEq);
        assert!(has_dotdoteq, "lexer should produce DotDotEq token for ..=");
    }
}

// ═══ Ownership & Type Inference Enhancement Tests ═══

#[cfg(test)]
mod ownership_enhanced_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;

    /// Helper: parse and run sema, expect it to succeed
    fn sema_ok(source: &str) {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept this program");
    }

    /// Helper: parse and run sema, expect ownership/type error
    fn sema_err(source: &str) -> Vec<crate::sema::SemaError> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect_err("sema should reject this program")
    }

    // ── Test 1: Move-after-use detection for non-Copy struct ──

    #[test]
    fn test_move_after_use_struct() {
        let source = r#"
struct Point { x: i64, y: i64 }

fn main() -> i64 {
    let p = Point { x: 1, y: 2 };
    let q = p;
    let r = p;
    return 0;
}
"#;
        let errors = sema_err(source);
        let has_move_error = errors.iter().any(|e| {
            if let crate::sema::SemaError::OwnershipError { message, .. } = e {
                message.contains("moved")
            } else {
                false
            }
        });
        assert!(has_move_error, "should detect use-after-move for struct: {:?}", errors);
    }

    // ── Test 2: Copy types do NOT move ──

    #[test]
    fn test_copy_types_no_move() {
        let source = r#"
fn main() -> i64 {
    let x: i64 = 5;
    let y = x;
    let z = x;
    return z;
}
"#;
        sema_ok(source);
    }

    // ── Test 3: Immutable borrow prevents mutation (API-level) ──

    #[test]
    fn test_immut_borrow_prevents_mutation() {
        use crate::sema::ownership::OwnershipChecker;
        use crate::lexer::Span;

        let mut oc = OwnershipChecker::new();
        let s1 = Span::new(0, 1, 1, 1);
        let s2 = Span::new(1, 2, 1, 2);
        let s3 = Span::new(2, 3, 1, 3);

        // Define a mutable, non-Copy variable
        oc.define_with_info("x", s1, true, false);

        // Immutably borrow x
        assert!(oc.check_borrow("x", s2).is_ok(), "immut borrow should succeed");

        // Try to mutate x while borrowed
        let result = oc.check_mutation("x", s3);
        assert!(result.is_err(), "mutation during immut borrow should fail");
        if let crate::sema::SemaError::OwnershipError { message, .. } = result.unwrap_err() {
            assert!(message.contains("immutably borrowed"),
                "error should mention immutable borrow: {}", message);
        }
    }

    // ── Test 4: Mutable borrow exclusivity ──

    #[test]
    fn test_mut_borrow_exclusivity() {
        use crate::sema::ownership::OwnershipChecker;
        use crate::lexer::Span;

        let mut oc = OwnershipChecker::new();
        let s1 = Span::new(0, 1, 1, 1);
        let s2 = Span::new(1, 2, 1, 2);
        let s3 = Span::new(2, 3, 1, 3);

        oc.define_with_info("x", s1, true, false);

        // First mutable borrow succeeds
        assert!(oc.check_mut_borrow("x", "r1", s2).is_ok());

        // Second mutable borrow fails (exclusivity)
        let result = oc.check_mut_borrow("x", "r2", s3);
        assert!(result.is_err(), "second mut borrow should fail");
        if let crate::sema::SemaError::OwnershipError { message, .. } = result.unwrap_err() {
            assert!(message.contains("already mutably borrowed"),
                "error should mention existing mutable borrow: {}", message);
        }
    }

    // ── Test 5: Function argument ownership transfer (non-Copy) ──

    #[test]
    fn test_fn_arg_ownership_transfer() {
        let source = r#"
struct Data { value: i64 }

fn consume(d: Data) -> i64 {
    return 0;
}

fn main() -> i64 {
    let d = Data { value: 42 };
    let r1 = consume(d);
    let r2 = consume(d);
    return 0;
}
"#;
        let errors = sema_err(source);
        let has_move_error = errors.iter().any(|e| {
            if let crate::sema::SemaError::OwnershipError { message, .. } = e {
                message.contains("moved")
            } else {
                false
            }
        });
        assert!(has_move_error, "should detect use-after-move for fn arg: {:?}", errors);
    }

    // ── Test 6: Return type mismatch detection ──

    #[test]
    fn test_return_type_mismatch() {
        let source = r#"
fn bad() -> i64 {
    return "hello";
}

fn main() -> i64 {
    return 0;
}
"#;
        let errors = sema_err(source);
        let has_type_error = errors.iter().any(|e| {
            matches!(e, crate::sema::SemaError::TypeError { .. })
        });
        assert!(has_type_error, "should detect return type mismatch: {:?}", errors);
    }

    // ── Test 7: Function call return type inference ──

    #[test]
    fn test_fn_call_return_type_inference() {
        let source = r#"
fn add(a: i64, b: i64) -> i64 {
    return a;
}

fn main() -> i64 {
    let x = add(1, 2);
    return x;
}
"#;
        sema_ok(source);
    }

    // ── Test 8: Immut-then-mut borrow conflict ──

    #[test]
    fn test_immut_then_mut_borrow_conflict() {
        use crate::sema::ownership::OwnershipChecker;
        use crate::lexer::Span;

        let mut oc = OwnershipChecker::new();
        let s1 = Span::new(0, 1, 1, 1);
        let s2 = Span::new(1, 2, 1, 2);
        let s3 = Span::new(2, 3, 1, 3);

        oc.define_with_info("x", s1, true, false);
        assert!(oc.check_borrow("x", s2).is_ok());

        // Mut borrow while immut borrow active should fail
        let result = oc.check_mut_borrow("x", "r", s3);
        assert!(result.is_err(), "mut borrow while immut borrowed should fail");
    }
}

// ═══ Mk.II Breakthrough Tests — Enum Payload + Match + Closure + Generic + Array Integration ═══

#[cfg(test)]
mod breakthrough_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::{HexaType, HexaOp};

    fn compile_to_ir(source: &str) -> Vec<crate::ir::HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    // ── Enum Payload Construction ──

    #[test]
    fn test_enum_payload_construction() {
        // Option::Some(42) should allocate tag + payload, NOT a function call
        let source = r#"
enum Option { Some(i64), None }

fn main() -> i64 {
    let x = Option::Some(42);
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        // Should have an Alloc with label "enum_variant_Option::Some"
        let variant_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc
                && i.label.as_ref().map_or(false, |l| l.contains("enum_variant_Option::Some")));
        assert!(variant_alloc.is_some(),
            "Option::Some(42) should emit enum_variant Alloc, not a Call");

        // Should NOT have a Call instruction for enum construction
        let has_enum_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Call
                && i.label.as_ref().map_or(false, |l| l.contains("Option::Some")));
        assert!(!has_enum_call,
            "Option::Some(42) should NOT lower to a Call instruction");

        // Should have a Store for the tag value
        let store_count = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Store)
            .count();
        assert!(store_count >= 2,
            "enum variant with payload should have at least 2 Stores (tag + payload), got {}",
            store_count);
    }

    #[test]
    fn test_enum_multiple_payload_fields() {
        let source = r#"
enum Shape { Circle(i64), Rect(i64, i64), Point }

fn main() -> i64 {
    let c = Shape::Circle(10);
    let r = Shape::Rect(6, 12);
    let p = Shape::Point;
    return 0;
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        // Circle should have tag 0
        let circle_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc
                && i.label.as_ref().map_or(false, |l| l.contains("enum_variant_Shape::Circle")));
        assert!(circle_alloc.is_some(), "should allocate enum_variant for Shape::Circle");

        // Rect should have tag 1 and TWO payload fields
        let rect_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc
                && i.label.as_ref().map_or(false, |l| l.contains("enum_variant_Shape::Rect")));
        assert!(rect_alloc.is_some(), "should allocate enum_variant for Shape::Rect");

        // Point (no payload) should be a plain tag Alloc
        let point_tag = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc
                && i.label.as_ref().map_or(false, |l| l.contains("enum_tag_Shape::Point")));
        assert!(point_tag.is_some(), "Shape::Point should be a simple enum_tag Alloc");
    }

    #[test]
    fn test_enum_payload_with_match() {
        // Full pipeline: construct enum with payload, then match on it
        let source = r#"
enum Option { Some(i64), None }

fn unwrap_or_default(opt: Option) -> i64 {
    return match opt {
        Option::Some(v) => v,
        Option::None => 0,
    };
}

fn main() -> i64 {
    let x = Option::Some(42);
    return unwrap_or_default(x);
}
"#;
        let funcs = compile_to_ir(source);
        assert!(funcs.len() >= 2, "should have main + unwrap_or_default");

        let unwrap_fn = funcs.iter().find(|f| f.name == "unwrap_or_default")
            .expect("should have unwrap_or_default");

        // Match should create multiple blocks
        assert!(unwrap_fn.blocks.len() >= 3,
            "match on Option should create 3+ blocks, got {}", unwrap_fn.blocks.len());

        // Should have variant_field Load for extracting payload
        let has_variant_field = unwrap_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Load
                && i.label.as_ref().map_or(false, |l| l.starts_with("variant_field_")));
        assert!(has_variant_field,
            "match on Option::Some(v) should emit variant_field Load for binding 'v'");
    }

    // ── Closure Block Body ──

    #[test]
    fn test_closure_with_block_body() {
        let source = r#"
fn main() -> i64 {
    let compute = |x: i64| -> i64 {
        let doubled = x * 2
        return doubled + 1
    }
    return 0
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept closure with block body");
    }

    #[test]
    fn test_closure_with_return_type_annotation() {
        let source = r#"
fn main() -> i64 {
    let add = |a: i64, b: i64| -> i64 { return a + b }
    return 0
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept closure with return type");
    }

    // ── Generic Call Syntax ──

    #[test]
    fn test_generic_call_turbofish() {
        let source = r#"
fn identity<T>(x: T) -> T {
    return x
}

fn main() -> i64 {
    return identity::<i64>(42)
}
"#;
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema should accept turbofish generic call");

        let funcs = compile_to_ir(source);
        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");

        let has_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Call);
        assert!(has_call, "turbofish call should lower to a Call instruction");
    }

    // ── Match on Integer Literals ──

    #[test]
    fn test_match_integer_literals() {
        let source = r#"
fn classify(x: i64) -> i64 {
    return match x {
        0 => 100,
        1 => 200,
        6 => 600,
        _ => 0,
    };
}

fn main() -> i64 {
    return classify(6);
}
"#;
        let funcs = compile_to_ir(source);
        let classify_fn = funcs.iter().find(|f| f.name == "classify")
            .expect("should have classify function");

        // Should create multiple blocks for integer match
        assert!(classify_fn.blocks.len() >= 4,
            "integer match with 3 literals + wildcard should have 4+ blocks, got {}",
            classify_fn.blocks.len());

        // Should have comparison branches
        let branch_count = classify_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .filter(|i| i.op == HexaOp::Branch)
            .count();
        assert!(branch_count >= 3,
            "3-literal match should have 3+ branches, got {}", branch_count);
    }

    // ── Array + For Integration ──

    #[test]
    fn test_array_for_loop_integration() {
        // Complete pipeline: array literal + for-in-array + accumulate
        let source = r#"
fn main() -> i64 {
    let nums = [1, 2, 3, 4, 5, 6]
    let mut total = 0
    for n in nums {
        total = total + n
    }
    return total
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");

        // Array alloc: 6 * 8 = 48 bytes
        let arr_alloc = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc && matches!(i.ty, HexaType::Array(_, 6)));
        assert!(arr_alloc.is_some(), "should allocate [i64; 6] array");
        assert_eq!(arr_alloc.unwrap().args[0], 48, "6-element array should be 48 bytes");

        // Loop should have header with comparison
        assert!(main_fn.blocks.len() >= 4,
            "array for-loop should create 4+ blocks, got {}",
            main_fn.blocks.len());
    }

    // ── Struct + Trait + Method Call Integration ──

    #[test]
    fn test_trait_method_dispatch_integration() {
        let source = r#"
trait Calculator {
    fn compute(&self) -> i64;
}

struct Adder { a: i64, b: i64 }

impl Calculator for Adder {
    fn compute(&self) -> i64 {
        return self.a + self.b;
    }
}

fn main() -> i64 {
    let adder = Adder { a: 6, b: 12 };
    return adder.compute();
}
"#;
        let funcs = compile_to_ir(source);

        // Should have Adder__compute function
        let compute_fn = funcs.iter().find(|f| f.name == "Adder__compute");
        assert!(compute_fn.is_some(), "should have mangled 'Adder__compute' function");

        // Compute function should have Add instruction (a + b)
        let has_add = compute_fn.unwrap().blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Add);
        assert!(has_add, "Adder.compute() should have Add for a + b");

        // main should dispatch to Adder__compute
        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");
        let has_dispatch = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Call
                && i.label.as_deref() == Some("Adder__compute"));
        assert!(has_dispatch, "main should call Adder__compute");
    }

    // ── Full Integration: All Features Together ──

    #[test]
    fn test_all_features_integration() {
        // enum + match + struct + array + for-loop + closure + generic in one program
        let source = r#"
enum Result { Ok(i64), Err(i64) }
struct Pair { first: i64, second: i64 }

fn identity<T>(x: T) -> T {
    return x
}

fn process(data: i64) -> Result {
    if data > 0 {
        return Result::Ok(data)
    }
    return Result::Err(0)
}

fn main() -> i64 {
    let p = Pair { first: 6, second: 12 }
    let arr = [1, 2, 3]
    let double = |x: i64| x * 2
    let id_val = identity(p.first)
    let r = process(id_val)
    return match r {
        Result::Ok(v) => v,
        Result::Err(e) => e,
    }
}
"#;
        let funcs = compile_to_ir(source);

        // Should have multiple functions: identity, process, main
        assert!(funcs.len() >= 3,
            "should have 3+ functions (identity, process, main), got {}",
            funcs.len());

        // main should have enum_variant Alloc (from process's Result::Ok/Err)
        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");
        assert!(main_fn.blocks.len() >= 2,
            "main with match should have 2+ blocks, got {}",
            main_fn.blocks.len());

        // process should have Branch (from if) and enum_variant construction
        let process_fn = funcs.iter().find(|f| f.name == "process").expect("process");
        let has_branch = process_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Branch);
        assert!(has_branch, "process() should have if-branch");

        let has_enum_variant = process_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Alloc
                && i.label.as_ref().map_or(false, |l| l.contains("enum_variant_")));
        assert!(has_enum_variant,
            "process() should construct enum variants with payload");
    }

    // ── Nested Match ──

    #[test]
    fn test_nested_enum_match() {
        let source = r#"
enum Outer { A(i64), B(i64) }

fn process(x: Outer) -> i64 {
    return match x {
        Outer::A(val) => val + 10,
        Outer::B(val) => val * 2,
    }
}

fn main() -> i64 {
    let x = Outer::A(6);
    return process(x);
}
"#;
        let funcs = compile_to_ir(source);
        let process_fn = funcs.iter().find(|f| f.name == "process")
            .expect("should have process function");

        // Should have multiple arm blocks
        let arm_block_count = process_fn.blocks.len();
        assert!(arm_block_count >= 4,
            "match with 2 variant arms should create 4+ blocks, got {}",
            arm_block_count);

        // Each arm should have its own computation (Add for A, Mul for B)
        let has_add = process_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Add);
        let has_mul = process_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Mul);
        assert!(has_add, "arm A should have Add");
        assert!(has_mul, "arm B should have Mul");
    }

    // ── For-in-Range with function calls in body ──

    #[test]
    fn test_for_loop_with_function_call() {
        let source = r#"
fn double(x: i64) -> i64 {
    return x * 2
}

fn main() -> i64 {
    let mut sum = 0
    for i in 0..6 {
        sum = sum + double(i)
    }
    return sum
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");

        // Should have Call to double inside loop body block
        let has_call = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Call && i.label.as_deref() == Some("double"));
        assert!(has_call, "for-loop body should contain Call to double()");
    }

    // ── Module + Use + Enum combo ──

    #[test]
    fn test_module_enum_combo() {
        let source = r#"
enum Status { Active, Inactive }

mod utils {
    pub fn check(s: Status) -> i64 {
        return 1
    }
}

fn main() -> i64 {
    let s = Status::Active;
    return utils::check(s)
}
"#;
        let funcs = compile_to_ir(source);

        let check_fn = funcs.iter().find(|f| f.name == "utils__check");
        assert!(check_fn.is_some(), "should have utils__check function");

        let main_fn = funcs.iter().find(|f| f.name == "main").expect("main");
        let has_tag = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.op == HexaOp::Alloc
                && i.label.as_deref() == Some("enum_tag_Status::Active"));
        assert!(has_tag, "should construct Status::Active tag");
    }

    // ── Closure capture + enum ──

    #[test]
    fn test_closure_captures_with_enum() {
        let source = r#"
enum Mode { Fast, Slow }

fn main() -> i64 {
    let m = Mode::Fast;
    let f = |x: i64| x + 1
    return 0
}
"#;
        let funcs = compile_to_ir(source);
        let main_fn = &funcs[0];

        // Should have both enum tag and closure env
        let has_tag = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.label.as_ref().map_or(false, |l| l.starts_with("enum_tag_")));
        let has_env = main_fn.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .any(|i| i.label.as_deref() == Some("closure_env"));
        assert!(has_tag, "should have enum tag allocation");
        assert!(has_env, "should have closure environment allocation");
    }
}

#[cfg(test)]
mod large_frame_tests {
    use crate::lexer;
    use crate::parser;
    use crate::sema;
    use crate::lower;
    use crate::ir::*;

    fn compile_to_ir(source: &str) -> Vec<HexaFunction> {
        let tokens = lexer::lex(source).expect("lex failed");
        let program = parser::parse(tokens).expect("parse failed");
        sema::analyze(&program).expect("sema failed");
        lower::lower_program(&program)
    }

    /// Test ARM64 codegen with a function that has 600+ local variables,
    /// producing a stack frame > 4096 bytes (exceeding ARM64 12-bit immediate).
    #[test]
    fn test_large_stack_frame_codegen() {
        // Generate a function with many local variables to force frame_size > 4095
        // Each SSA slot = 8 bytes, so 600 vars = 4800+ bytes of slots
        let mut source = String::from("fn big_frame() -> i64 {\n");
        for i in 0..600 {
            source.push_str(&format!("    let v{} = {}\n", i, i));
        }
        source.push_str("    return v599\n}\n");
        source.push_str("fn main() -> i64 { return big_frame() }\n");

        let funcs = compile_to_ir(&source);
        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);

        // Verify no bare `sub sp, sp, #LARGE` or `add sp, sp, #LARGE` with imm > 4095
        // Instead should use movz + sub/add sp, sp, xN pattern
        for line in asm.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("sub sp, sp, #") {
                if let Ok(imm) = rest.parse::<usize>() {
                    assert!(imm <= 4095,
                        "sub sp, sp, #{} exceeds 12-bit immediate limit (max 4095)", imm);
                }
            }
            if let Some(rest) = trimmed.strip_prefix("add sp, sp, #") {
                if let Ok(imm) = rest.parse::<usize>() {
                    assert!(imm <= 4095,
                        "add sp, sp, #{} exceeds 12-bit immediate limit (max 4095)", imm);
                }
            }
        }

        // Should use movz + sub sp, sp, x16 pattern for the large frame
        assert!(asm.contains("sub sp, sp, x16") || asm.contains("movz x16"),
            "Large frame should use register-based sub sp via movz+sub pattern");

        // Basic sanity: should have function labels
        assert!(asm.contains("_big_frame:"), "Should have big_frame label");
        assert!(asm.contains("_main:"), "Should have main label");
    }

    /// Test that small frames still use the efficient immediate form
    #[test]
    fn test_small_stack_frame_uses_immediate() {
        let source = r#"
fn small() -> i64 {
    let a = 1
    let b = 2
    return a + b
}

fn main() -> i64 { return small() }
"#;
        let funcs = compile_to_ir(source);
        let asm = crate::codegen::arm64_asm::emit_program_asm(&funcs);

        // Small frame should NOT use the movz+sub pattern
        // (the stp pre-indexed form handles it)
        let has_movz_sub = asm.contains("movz x16") && asm.contains("sub sp, sp, x16");
        assert!(!has_movz_sub,
            "Small frame should use stp pre-indexed, not movz+sub pattern");
    }
}
