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

/// Struct field definition: (field_name, field_type)
pub type StructFieldDef = (String, HexaType);

/// Lowering context — tracks SSA register/block counters and variable bindings
pub struct LowerContext {
    pub functions: Vec<HexaFunction>,
    reg_counter: usize,
    block_counter: usize,
    /// Variable name -> SSA register holding its current value
    pub vars: HashMap<String, usize>,
    /// String constant pool for the current function being lowered
    pub string_pool: Vec<String>,
    /// Struct definitions: struct_name -> Vec<(field_name, field_type)>
    pub struct_defs: HashMap<String, Vec<StructFieldDef>>,
    /// Variable name -> struct type name (for field resolution)
    pub var_struct_types: HashMap<String, String>,
    /// Enum definitions: enum_name -> [(variant_name, optional_payload_types)]
    pub enum_defs: HashMap<String, Vec<(String, Option<Vec<HexaType>>)>>,
    /// Extra blocks created during expression lowering (e.g., match arms)
    /// Drained after each statement/block lowering cycle.
    pub pending_blocks: Vec<HexaBlock>,
    /// Method dispatch: (type_name, method_name) -> mangled function name
    pub method_dispatch: HashMap<(String, String), String>,
    /// Closure IR info collected during lowering (for lambda lifting)
    pub closure_fns: Vec<closure::ClosureIR>,
    /// Counter for generating unique closure function names
    closure_counter: usize,
    /// Generic function definitions for monomorphization
    pub generic_fn_defs: HashMap<String, ast::FnDecl>,
    /// Already monomorphized instances
    pub mono_instances: HashMap<String, bool>,
}

impl LowerContext {
    pub fn new() -> Self {
        LowerContext {
            functions: Vec::new(),
            reg_counter: 0,
            block_counter: 0,
            vars: HashMap::new(),
            string_pool: Vec::new(),
            struct_defs: HashMap::new(),
            var_struct_types: HashMap::new(),
            enum_defs: HashMap::new(),
            pending_blocks: Vec::new(),
            method_dispatch: HashMap::new(),
            closure_fns: Vec::new(),
            closure_counter: 0,
            generic_fn_defs: HashMap::new(),
            mono_instances: HashMap::new(),
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

    /// Intern a string literal in the current function's string pool.
    /// Returns the pool index.
    pub fn intern_string(&mut self, s: &str) -> usize {
        // Check for existing identical string
        if let Some(idx) = self.string_pool.iter().position(|existing| existing == s) {
            return idx;
        }
        let idx = self.string_pool.len();
        self.string_pool.push(s.to_string());
        idx
    }

    /// Register a struct definition
    pub fn register_struct(&mut self, name: &str, fields: Vec<StructFieldDef>) {
        self.struct_defs.insert(name.to_string(), fields);
    }

    /// Look up struct field: returns (field_index, byte_offset, field_type)
    pub fn lookup_struct_field(&self, struct_name: &str, field_name: &str) -> Option<(usize, usize, HexaType)> {
        let fields = self.struct_defs.get(struct_name)?;
        let mut byte_offset = 0usize;
        for (i, (fname, fty)) in fields.iter().enumerate() {
            if fname == field_name {
                return Some((i, byte_offset, fty.clone()));
            }
            byte_offset += fty.size_bytes();
        }
        None
    }

    /// Get total byte size of a struct
    pub fn struct_size(&self, struct_name: &str) -> usize {
        self.struct_defs.get(struct_name)
            .map(|fields| fields.iter().map(|(_, ty)| ty.size_bytes()).sum())
            .unwrap_or(0)
    }

    /// Bind a variable to a struct type name (for field access resolution)
    pub fn bind_var_struct_type(&mut self, var_name: &str, struct_name: &str) {
        self.var_struct_types.insert(var_name.to_string(), struct_name.to_string());
    }

    /// Look up which struct type a variable has
    pub fn lookup_var_struct_type(&self, var_name: &str) -> Option<&String> {
        self.var_struct_types.get(var_name)
    }

    /// Look up an enum variant by qualified name "EnumName::VariantName".
    /// Returns (tag_index, optional_payload_types).
    pub fn lookup_enum_variant(&self, qualified: &str) -> Option<(usize, Option<Vec<HexaType>>)> {
        let parts: Vec<&str> = qualified.splitn(2, "::").collect();
        if parts.len() != 2 {
            return None;
        }
        let (enum_name, variant_name) = (parts[0], parts[1]);
        let variants = self.enum_defs.get(enum_name)?;
        for (i, (vname, payload)) in variants.iter().enumerate() {
            if vname == variant_name {
                return Some((i, payload.clone()));
            }
        }
        None
    }

    /// Get next unique closure ID
    pub fn next_closure_id(&mut self) -> usize {
        let id = self.closure_counter;
        self.closure_counter += 1;
        id
    }
}

/// Lower an entire program to a list of IR functions
pub fn lower_program(program: &ast::Program) -> Vec<HexaFunction> {
    let mut ctx = LowerContext::new();

    // First pass: register all struct and enum declarations
    for decl in &program.decls {
        match decl {
            ast::Decl::StructDecl(s) => {
                let fields: Vec<StructFieldDef> = s.fields.iter()
                    .map(|(name, ty_expr)| (name.clone(), lower_type_expr(ty_expr)))
                    .collect();
                ctx.register_struct(&s.name, fields);
            }
            ast::Decl::EnumDecl(e) => {
                let variants: Vec<(String, Option<Vec<HexaType>>)> = e.variants.iter()
                    .map(|(name, payload)| {
                        let payload_tys = payload.as_ref().map(|tys|
                            tys.iter().map(|t| lower_type_expr(t)).collect()
                        );
                        (name.clone(), payload_tys)
                    })
                    .collect();
                ctx.enum_defs.insert(e.name.clone(), variants);
            }
            ast::Decl::ModuleDecl(m) => {
                register_module_types(&mut ctx, &m.decls);
            }
            ast::Decl::ImplBlock(ib) => {
                for method in &ib.methods {
                    let mangled = format!("{}__{}", ib.target_type, method.name);
                    ctx.method_dispatch.insert(
                        (ib.target_type.clone(), method.name.clone()),
                        mangled,
                    );
                }
            }
            _ => {}
        }
    }

    // Collect generic function definitions
    for decl in &program.decls {
        if let ast::Decl::FnDecl(f) = decl {
            if !f.type_params.is_empty() {
                ctx.generic_fn_defs.insert(f.name.clone(), f.clone());
            }
        }
    }

    // Second pass: lower function bodies
    for decl in &program.decls {
        match decl {
            ast::Decl::FnDecl(f) => {
                if f.type_params.is_empty() {
                    let func = lower_fn(&mut ctx, f);
                    ctx.functions.push(func);
                }
            }
            ast::Decl::ModuleDecl(m) => {
                lower_module_fns(&mut ctx, &m.name, &m.decls);
            }
            ast::Decl::ImplBlock(ib) => {
                for method in &ib.methods {
                    let mangled_name = format!("{}__{}", ib.target_type, method.name);
                    let mut mangled_method = method.clone();
                    mangled_method.name = mangled_name;
                    let func = lower_fn(&mut ctx, &mangled_method);
                    ctx.functions.push(func);
                }
            }
            // struct/enum/type alias/trait/use handled at type level
            _ => {}
        }
    }
    // Monomorphize generic functions (type params erased to Any)
    let generic_defs: Vec<ast::FnDecl> = ctx.generic_fn_defs.values().cloned().collect();
    for gf in &generic_defs {
        if !ctx.mono_instances.contains_key(&gf.name) {
            let func = lower_fn(&mut ctx, gf);
            ctx.functions.push(func);
            ctx.mono_instances.insert(gf.name.clone(), true);
        }
    }

    ctx.functions
}

/// Lower a single function declaration to HexaFunction
fn lower_fn(ctx: &mut LowerContext, f: &ast::FnDecl) -> HexaFunction {
    // Reset per-function state (struct_defs preserved across functions)
    ctx.reg_counter = 0;
    ctx.block_counter = 0;
    ctx.vars.clear();
    ctx.var_struct_types.clear();
    ctx.string_pool.clear();

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
        string_pool: ctx.string_pool.clone(),
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
        ast::TypeExpr::TypeParam(_, _) => HexaType::Any, // generic → Any until monomorphization
    }
}

/// Register struct/enum types from inside a module
fn register_module_types(ctx: &mut LowerContext, decls: &[ast::Decl]) {
    for decl in decls {
        match decl {
            ast::Decl::StructDecl(s) => {
                let fields: Vec<StructFieldDef> = s.fields.iter()
                    .map(|(name, ty_expr)| (name.clone(), lower_type_expr(ty_expr)))
                    .collect();
                ctx.register_struct(&s.name, fields);
            }
            ast::Decl::EnumDecl(e) => {
                let variants: Vec<(String, Option<Vec<HexaType>>)> = e.variants.iter()
                    .map(|(name, payload)| {
                        let payload_tys = payload.as_ref().map(|tys|
                            tys.iter().map(|t| lower_type_expr(t)).collect()
                        );
                        (name.clone(), payload_tys)
                    })
                    .collect();
                ctx.enum_defs.insert(e.name.clone(), variants);
            }
            ast::Decl::ModuleDecl(m) => {
                register_module_types(ctx, &m.decls);
            }
            _ => {}
        }
    }
}

/// Lower functions inside a module with mangled names: mod__fn
fn lower_module_fns(ctx: &mut LowerContext, mod_name: &str, decls: &[ast::Decl]) {
    for decl in decls {
        match decl {
            ast::Decl::FnDecl(f) => {
                let mangled_name = format!("{}__{}", mod_name, f.name);
                let mut mangled_fn = f.clone();
                mangled_fn.name = mangled_name;
                let func = lower_fn(ctx, &mangled_fn);
                ctx.functions.push(func);
            }
            ast::Decl::ModuleDecl(m) => {
                let nested_name = format!("{}__{}", mod_name, m.name);
                lower_module_fns(ctx, &nested_name, &m.decls);
            }
            _ => {}
        }
    }
}
