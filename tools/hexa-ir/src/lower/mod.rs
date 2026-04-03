/// AST -> HEXA-IR Lowering — n=6 sub-modules
///
/// Transforms parsed + type-checked AST into SSA-form IR.
/// Each expression yields an SSA register; control flow creates blocks.

pub mod expr_lower;
pub mod stmt_lower;
pub mod pattern;
pub mod closure;
pub mod proof_emit;

use crate::ir::*;
use crate::parser::ast;
use std::collections::HashMap;

/// Lowering context — tracks SSA register/block counters and variable bindings
pub struct LowerContext {
    pub functions: Vec<HexaFunction>,
    reg_counter: usize,
    block_counter: usize,
    /// Variable name -> SSA register holding its current value
    pub vars: HashMap<String, usize>,
}

impl LowerContext {
    pub fn new() -> Self {
        LowerContext {
            functions: Vec::new(),
            reg_counter: 0,
            block_counter: 0,
            vars: HashMap::new(),
        }
    }

    pub fn fresh_reg(&mut self) -> usize {
        let r = self.reg_counter;
        self.reg_counter += 1;
        r
    }

    pub fn fresh_block(&mut self) -> usize {
        let b = self.block_counter;
        self.block_counter += 1;
        b
    }

    /// Bind a variable name to an SSA register
    pub fn bind_var(&mut self, name: &str, reg: usize) {
        self.vars.insert(name.to_string(), reg);
    }

    /// Look up the SSA register for a variable
    pub fn lookup_var(&self, name: &str) -> Option<usize> {
        self.vars.get(name).copied()
    }
}

/// Lower an entire program to a list of IR functions
pub fn lower_program(program: &ast::Program) -> Vec<HexaFunction> {
    let mut ctx = LowerContext::new();
    for decl in &program.decls {
        match decl {
            ast::Decl::FnDecl(f) => {
                let func = lower_fn(&mut ctx, f);
                ctx.functions.push(func);
            }
            // struct/enum/type alias handled at type level — no IR emitted
            _ => {}
        }
    }
    ctx.functions
}

/// Lower a single function declaration to HexaFunction
fn lower_fn(ctx: &mut LowerContext, f: &ast::FnDecl) -> HexaFunction {
    // Reset per-function state
    ctx.reg_counter = 0;
    ctx.block_counter = 0;
    ctx.vars.clear();

    // Create parameter registers and bind them
    let mut params = Vec::new();
    for (name, ty_expr) in &f.params {
        let reg = ctx.fresh_reg();
        ctx.bind_var(name, reg);
        let ty = lower_type_expr(ty_expr);
        params.push((name.clone(), ty));
    }

    // Create entry block
    let entry_id = ctx.fresh_block();
    let mut entry_block = HexaBlock {
        id: entry_id,
        instrs: Vec::new(),
        successors: Vec::new(),
    };

    // Lower the function body statements
    let mut blocks = Vec::new();
    stmt_lower::lower_block(ctx, &mut entry_block, &f.body, &mut blocks);
    blocks.push(entry_block);
    // Sort blocks by id so the entry block (bb0) is always first
    blocks.sort_by_key(|b| b.id);

    // Determine return type
    let ret_ty = match &f.ret_ty {
        Some(ty_expr) => lower_type_expr(ty_expr),
        None => HexaType::Void,
    };

    HexaFunction {
        name: f.name.clone(),
        blocks,
        params,
        ret_ty,
    }
}

/// Convert AST type expression to IR type
pub fn lower_type_expr(ty: &ast::TypeExpr) -> HexaType {
    match ty {
        ast::TypeExpr::Named(name, _) => match name.as_str() {
            "i64" | "int" => HexaType::I64,
            "f64" | "float" => HexaType::F64,
            "bool" => HexaType::Bool,
            "char" => HexaType::Char,
            "str" | "string" => HexaType::Str,
            "byte" | "u8" => HexaType::Byte,
            "void" | "()" => HexaType::Void,
            _ => HexaType::Any, // user-defined struct/enum → resolve later
        },
        ast::TypeExpr::Array(inner, size, _) => {
            HexaType::Array(Box::new(lower_type_expr(inner)), *size)
        }
        ast::TypeExpr::Fn(param_tys, ret_ty, _) => {
            let params: Vec<HexaType> = param_tys.iter().map(|t| lower_type_expr(t)).collect();
            let ret = lower_type_expr(ret_ty);
            HexaType::Fn(params, Box::new(ret))
        }
    }
}
