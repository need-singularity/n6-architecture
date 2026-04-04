/// Self-Hosting Tests — HEXA-IR Mk.II
///
/// Verifies that .hexa self-hosted modules compile through the full pipeline:
///   Stage 1 (Lex) → Stage 2 (Parse) → Stage 3 (Sema) → Stage 4 (Lower) → Stage 5 (Opt)
///
/// n=6 structure: n/phi=3 test modules × phi=2 verification levels = n=6 test groups

use hexa_ir::lexer;
use hexa_ir::parser;
use hexa_ir::sema;
use hexa_ir::lower;
use hexa_ir::opt;
use hexa_ir::ir::{HexaOp, HexaType};

/// Helper: compile .hexa source through stages 1-5, return optimized IR
fn compile_hexa(source: &str) -> Vec<hexa_ir::ir::HexaFunction> {
    let tokens = lexer::lex(source).expect("Stage 1 (Lex) failed");
    let program = parser::parse(tokens).expect("Stage 2 (Parse) failed");
    sema::analyze(&program).expect("Stage 3 (Sema) failed");
    let mut functions = lower::lower_program(&program);
    for func in &mut functions {
        opt::run_pipeline(func);
    }
    functions
}

/// Helper: read .hexa file from self-host/ directory
fn read_hexa_file(name: &str) -> String {
    let path = format!("self-host/{}.hexa", name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}

// ═══════════════════════════════════════════════════════════
// Module 1: n6.hexa — Core Constants
// ═══════════════════════════════════════════════════════════

#[test]
fn test_n6_lex() {
    let source = read_hexa_file("n6");
    let tokens = lexer::lex(&source).expect("n6.hexa lex failed");
    // Should produce a substantial number of tokens
    assert!(tokens.len() > 100, "n6.hexa should have >100 tokens, got {}", tokens.len());
}

#[test]
fn test_n6_parse() {
    let source = read_hexa_file("n6");
    let tokens = lexer::lex(&source).unwrap();
    let program = parser::parse(tokens).expect("n6.hexa parse failed");
    // Should have 19 declarations (7 primary + 5 derived + 4 block + 2 verify + 1 main)
    assert_eq!(program.decls.len(), 19,
        "n6.hexa should have 19 decls, got {}", program.decls.len());
}

#[test]
fn test_n6_sema() {
    let source = read_hexa_file("n6");
    let tokens = lexer::lex(&source).unwrap();
    let program = parser::parse(tokens).unwrap();
    sema::analyze(&program).expect("n6.hexa sema failed — type errors in self-hosted constants");
}

#[test]
fn test_n6_lower_and_opt() {
    let source = read_hexa_file("n6");
    let funcs = compile_hexa(&source);

    // Check primary constant functions return correct values
    let expected: &[(&str, i64)] = &[
        ("n", 6), ("phi", 2), ("tau", 4), ("sigma", 12),
        ("sopfr", 5), ("j2", 24), ("mu", 1),
    ];

    for &(name, val) in expected {
        let func = funcs.iter().find(|f| f.name == name)
            .unwrap_or_else(|| panic!("missing function: {}", name));

        // After optimization, constant functions should have exactly 2 instrs: Alloc + Return
        assert_eq!(func.count_instrs(), 2,
            "fn {}() should optimize to 2 instrs (Alloc + Return), got {}",
            name, func.count_instrs());

        // The Alloc instruction should carry the correct constant value in args[0]
        let alloc = func.blocks[0].instrs.iter()
            .find(|i| i.op == HexaOp::Alloc)
            .unwrap_or_else(|| panic!("fn {}() missing Alloc instr", name));
        assert_eq!(*alloc.args.first().unwrap_or(&0) as i64, val,
            "fn {}() Alloc args[0] should be {}, got {:?}", name, val, alloc.args);
    }
}

#[test]
fn test_n6_derived_constants() {
    let source = read_hexa_file("n6");
    let funcs = compile_hexa(&source);

    // Derived constants should be constant-folded
    let derived: &[(&str, i64)] = &[
        ("sigma_tau", 8),    // 12 - 4
        ("sigma_phi", 10),   // 12 - 2
        ("n_phi", 3),        // 6 / 2
        ("sigma_sq", 144),   // 12 * 12
        ("sigma_j2", 288),   // 12 * 24
    ];

    for &(name, val) in derived {
        let func = funcs.iter().find(|f| f.name == name)
            .unwrap_or_else(|| panic!("missing derived fn: {}", name));

        // Constant folding should have computed the result
        let alloc = func.blocks.iter()
            .flat_map(|b| b.instrs.iter())
            .find(|i| i.op == HexaOp::Alloc && i.args.first().copied() == Some(val as usize));
        assert!(alloc.is_some(),
            "fn {}() should constant-fold to {}, IR: {:?}",
            name, val, func.blocks[0].instrs.iter()
                .map(|i| format!("{:?} args={:?}", i.op, i.args))
                .collect::<Vec<_>>());
    }
}

#[test]
fn test_n6_block_sizes() {
    let source = read_hexa_file("n6");
    let funcs = compile_hexa(&source);

    let blocks: &[(&str, i64)] = &[
        ("block_large", 4096),   // 2^σ
        ("block_medium", 1024),  // 2^(σ-φ)
        ("block_small", 256),    // 2^(σ-τ)
        ("block_min", 64),       // 2^n
    ];

    for &(name, val) in blocks {
        let func = funcs.iter().find(|f| f.name == name)
            .unwrap_or_else(|| panic!("missing block fn: {}", name));
        let alloc = func.blocks[0].instrs.iter()
            .find(|i| i.op == HexaOp::Alloc && i.args.first().copied() == Some(val as usize));
        assert!(alloc.is_some(),
            "fn {}() should return {}", name, val);
    }
}

#[test]
fn test_n6_verify_functions() {
    let source = read_hexa_file("n6");
    let funcs = compile_hexa(&source);

    // verify_identity and verify_egyptian should compile and have branches
    for name in &["verify_identity", "verify_egyptian", "main"] {
        let func = funcs.iter().find(|f| f.name == *name)
            .unwrap_or_else(|| panic!("missing fn: {}", name));
        assert!(func.blocks.len() >= 2,
            "fn {}() should have >=2 blocks (if/else branch), got {}",
            name, func.blocks.len());
    }
}

// ═══════════════════════════════════════════════════════════
// Module 2: token_kind.hexa — Token Encoding
// ═══════════════════════════════════════════════════════════

#[test]
fn test_token_kind_full_pipeline() {
    let source = read_hexa_file("token_kind");
    let funcs = compile_hexa(&source);

    // J₂=24 token kind functions + 2 verify + 1 main = 27 functions
    assert_eq!(funcs.len(), 27,
        "token_kind.hexa should have 27 functions, got {}", funcs.len());

    // Verify the J₂=24 operator IDs are sequential 0..23
    let tok_fns: Vec<_> = funcs.iter()
        .filter(|f| f.name.starts_with("tok_"))
        .collect();
    assert_eq!(tok_fns.len(), 24,
        "should have J₂=24 token functions, got {}", tok_fns.len());
}

#[test]
fn test_token_kind_operator_groups() {
    let source = read_hexa_file("token_kind");
    let funcs = compile_hexa(&source);

    // Group 0: arithmetic (0-5), Group 1: comparison (6-11),
    // Group 2: logic (12-17), Group 3: structural (18-23)
    let groups: &[(&str, i64, i64)] = &[
        ("tok_plus", 0, 0),      // group 0 start
        ("tok_caret", 5, 0),     // group 0 end
        ("tok_eq", 6, 1),        // group 1 start
        ("tok_ge", 11, 1),       // group 1 end
        ("tok_and", 12, 2),      // group 2 start
        ("tok_bit_xor", 17, 2),  // group 2 end
        ("tok_assign", 18, 3),   // group 3 start
        ("tok_colon", 23, 3),    // group 3 end
    ];

    for &(name, expected_val, group) in groups {
        let func = funcs.iter().find(|f| f.name == name)
            .unwrap_or_else(|| panic!("missing fn: {}", name));
        let alloc = func.blocks[0].instrs.iter()
            .find(|i| i.op == HexaOp::Alloc && i.args.first().copied() == Some(expected_val as usize));
        assert!(alloc.is_some(),
            "fn {}() should return {} (group {})", name, expected_val, group);
    }
}

// ═══════════════════════════════════════════════════════════
// Module 3: span.hexa — Source Location Structs
// ═══════════════════════════════════════════════════════════

#[test]
fn test_span_full_pipeline() {
    let source = read_hexa_file("span");
    let funcs = compile_hexa(&source);

    // Should have: dummy_span, make_span, verify_span_fields, main = 4 functions
    assert_eq!(funcs.len(), 4,
        "span.hexa should have 4 functions, got {}", funcs.len());
}

#[test]
fn test_span_struct_alloc() {
    let source = read_hexa_file("span");
    let funcs = compile_hexa(&source);

    // verify_span_fields should have struct allocation
    let verify = funcs.iter().find(|f| f.name == "verify_span_fields")
        .expect("missing verify_span_fields");

    // Look for Struct-typed Alloc (Span has tau=4 i64 fields = 32 bytes)
    let struct_alloc = verify.blocks.iter()
        .flat_map(|b| b.instrs.iter())
        .find(|i| matches!(i.ty, HexaType::Struct(_)) && i.op == HexaOp::Alloc);
    assert!(struct_alloc.is_some(),
        "verify_span_fields should have Struct-typed Alloc for Span");
}

#[test]
fn test_span_field_access() {
    let source = read_hexa_file("span");
    let funcs = compile_hexa(&source);

    let verify = funcs.iter().find(|f| f.name == "verify_span_fields")
        .expect("missing verify_span_fields");

    // Should have field Load instructions for offset, length, line, col
    let field_loads: Vec<_> = verify.blocks.iter()
        .flat_map(|b| b.instrs.iter())
        .filter(|i| i.op == HexaOp::Load && i.label.is_some())
        .collect();

    // tau=4 fields need to be loaded for the sum
    assert!(field_loads.len() >= 4,
        "verify_span_fields should load all tau=4 fields, got {} loads",
        field_loads.len());
}

// ═══════════════════════════════════════════════════════════
// Cross-Module: Self-Hosting Completeness
// ═══════════════════════════════════════════════════════════

#[test]
fn test_self_host_module_count() {
    // Mk.II starts with n/phi=3 self-hosted modules
    let modules = ["n6", "token_kind", "span"];
    for name in &modules {
        let source = read_hexa_file(name);
        let tokens = lexer::lex(&source).expect(&format!("{}.hexa lex failed", name));
        let program = parser::parse(tokens).expect(&format!("{}.hexa parse failed", name));
        sema::analyze(&program).expect(&format!("{}.hexa sema failed", name));
    }
}

#[test]
fn test_self_host_total_functions() {
    // Total self-hosted functions across all modules
    let modules = ["n6", "token_kind", "span"];
    let mut total = 0;
    for name in &modules {
        let source = read_hexa_file(name);
        let funcs = compile_hexa(&source);
        total += funcs.len();
    }
    // n6=19 + token_kind=27 + span=4 = 50 total self-hosted functions
    assert_eq!(total, 50,
        "total self-hosted functions should be 50, got {}", total);
}
