/// Type checking — Hindley-Milner Layer 1
///
/// Walks the AST, checks type consistency of expressions and statements,
/// and resolves type names to HexaType.

use std::collections::HashMap;
use crate::lexer::Span;
use crate::ir::HexaType;
use crate::parser::ast::*;
use crate::parser::ast::Pattern;
use super::resolve::{Resolver, VarInfo};
use super::error::SemaError;

/// Type checker state
pub struct TypeChecker {
    resolver: Resolver,
    /// Return type of the current function (for return-statement checking)
    current_ret_ty: Option<HexaType>,
    /// Struct definitions: struct_name -> Vec<(field_name, HexaType)>
    struct_defs: HashMap<String, Vec<(String, HexaType)>>,
    /// Trait definitions: trait_name -> list of required method names
    trait_defs: HashMap<String, Vec<String>>,
    /// Impl table: (trait_name, type_name) -> method names
    impl_table: HashMap<(String, String), Vec<String>>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut tc = TypeChecker {
            resolver: Resolver::new(),
            current_ret_ty: None,
            struct_defs: HashMap::new(),
            trait_defs: HashMap::new(),
            impl_table: HashMap::new(),
        };
        // Register built-in functions (file I/O, memory, print)
        // These are special-cased in codegen as inline syscalls
        let builtins = [
            ("print",      "fn -> i64"),
            ("file_open",  "fn -> i64"),
            ("file_read",  "fn -> i64"),
            ("file_write", "fn -> i64"),
            ("file_close", "fn -> i64"),
            ("heap_alloc", "fn -> i64"),
            ("heap_free",  "fn -> i64"),
        ];
        let builtin_span = Span::dummy();
        for (name, ty) in builtins {
            let _ = tc.resolver.define(VarInfo {
                name: name.to_string(),
                ty_name: ty.to_string(),
                mutable: false,
                defined_at: builtin_span,
            });
        }
        tc
    }

    /// Check an entire program: all declarations
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<SemaError>> {
        let mut errors = Vec::new();

        // First pass: register all top-level function/struct/enum names
        for decl in &program.decls {
            match decl {
                Decl::FnDecl(f) => {
                    let ret_name = f.ret_ty.as_ref()
                        .map(type_expr_to_name)
                        .unwrap_or_else(|| "void".to_string());
                    if let Err(e) = self.resolver.define(VarInfo {
                        name: f.name.clone(),
                        ty_name: format!("fn -> {}", ret_name),
                        mutable: false,
                        defined_at: f.span,
                    }) {
                        errors.push(e);
                    }
                }
                Decl::StructDecl(s) => {
                    // Register struct in resolver
                    if let Err(e) = self.resolver.define(VarInfo {
                        name: s.name.clone(),
                        ty_name: "struct".to_string(),
                        mutable: false,
                        defined_at: s.span,
                    }) {
                        errors.push(e);
                    }
                    // Register struct field definitions for type checking
                    let fields: Vec<(String, HexaType)> = s.fields.iter()
                        .map(|(name, ty_expr)| (name.clone(), resolve_type_name(&type_expr_to_name(ty_expr))))
                        .collect();
                    self.struct_defs.insert(s.name.clone(), fields);
                }
                Decl::EnumDecl(en) => {
                    if let Err(e) = self.resolver.define(VarInfo {
                        name: en.name.clone(),
                        ty_name: "enum".to_string(),
                        mutable: false,
                        defined_at: en.span,
                    }) {
                        errors.push(e);
                    }
                    // Register each variant as "EnumName::VariantName"
                    for (vname, _) in &en.variants {
                        let qualified = format!("{}::{}", en.name, vname);
                        if let Err(e) = self.resolver.define(VarInfo {
                            name: qualified,
                            ty_name: en.name.clone(),
                            mutable: false,
                            defined_at: en.span,
                        }) {
                            errors.push(e);
                        }
                    }
                }
                Decl::TypeAlias(t) => {
                    if let Err(e) = self.resolver.define(VarInfo {
                        name: t.name.clone(),
                        ty_name: type_expr_to_name(&t.ty),
                        mutable: false,
                        defined_at: t.span,
                    }) {
                        errors.push(e);
                    }
                }
                Decl::TraitDecl(td) => {
                    let method_names: Vec<String> = td.methods.iter()
                        .map(|m| m.name.clone()).collect();
                    self.trait_defs.insert(td.name.clone(), method_names);
                    if let Err(e) = self.resolver.define(VarInfo {
                        name: td.name.clone(),
                        ty_name: "trait".to_string(),
                        mutable: false,
                        defined_at: td.span,
                    }) { errors.push(e); }
                }
                Decl::ImplBlock(ib) => {
                    if !self.trait_defs.contains_key(&ib.trait_name) {
                        errors.push(SemaError::UndefinedName {
                            span: ib.span,
                            name: format!("trait '{}'", ib.trait_name),
                        });
                    } else {
                        let required = self.trait_defs.get(&ib.trait_name).cloned().unwrap_or_default();
                        let impl_names: Vec<&str> = ib.methods.iter().map(|m| m.name.as_str()).collect();
                        for req in &required {
                            if !impl_names.contains(&req.as_str()) {
                                errors.push(SemaError::TypeError {
                                    span: ib.span,
                                    expected: format!("method '{}' required by trait '{}'", req, ib.trait_name),
                                    found: "not implemented".to_string(),
                                });
                            }
                        }
                    }
                    for method in &ib.methods {
                        let mangled = format!("{}__{}", ib.target_type, method.name);
                        let ret_name = method.ret_ty.as_ref().map(type_expr_to_name).unwrap_or_else(|| "void".to_string());
                        if let Err(e) = self.resolver.define(VarInfo {
                            name: mangled,
                            ty_name: format!("fn -> {}", ret_name),
                            mutable: false,
                            defined_at: method.span,
                        }) { errors.push(e); }
                    }
                }
                Decl::ModuleDecl(m) => {
                    self.register_module_symbols(&m.name, &m.decls, &mut errors);
                }
                Decl::UseDecl(u) => {
                    if u.path.len() >= 2 {
                        let qualified = u.path.join("::");
                        let short_name = u.path.last().unwrap().clone();
                        if self.resolver.is_defined(&qualified) {
                            let info = self.resolver.lookup(&qualified, u.span)
                                .ok().cloned();
                            if let Some(info) = info {
                                let _ = self.resolver.define(VarInfo {
                                    name: short_name,
                                    ty_name: info.ty_name.clone(),
                                    mutable: false,
                                    defined_at: u.span,
                                });
                            }
                        } else {
                            errors.push(SemaError::UndefinedName {
                                span: u.span,
                                name: qualified,
                            });
                        }
                    }
                }
            }
        }

        // Second pass: check function bodies (including module functions)
        for decl in &program.decls {
            match decl {
                Decl::FnDecl(f) => {
                    if let Err(mut errs) = self.check_fn(f) {
                        errors.append(&mut errs);
                    }
                }
                Decl::ModuleDecl(m) => {
                    self.check_module_fns(&m.name, &m.decls, &mut errors);
                }
                Decl::ImplBlock(ib) => {
                    for method in &ib.methods {
                        if let Err(mut errs) = self.check_fn(method) {
                            errors.append(&mut errs);
                        }
                    }
                }
                _ => {}
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Register symbols from a module declaration with qualified names.
    /// Only pub items are visible from outside the module.
    fn register_module_symbols(&mut self, mod_name: &str, decls: &[Decl], errors: &mut Vec<SemaError>) {
        for decl in decls {
            match decl {
                Decl::FnDecl(f) => {
                    if f.vis == Visibility::Public {
                        let qualified = format!("{}::{}", mod_name, f.name);
                        let ret_name = f.ret_ty.as_ref()
                            .map(type_expr_to_name)
                            .unwrap_or_else(|| "void".to_string());
                        if let Err(e) = self.resolver.define(VarInfo {
                            name: qualified,
                            ty_name: format!("fn -> {}", ret_name),
                            mutable: false,
                            defined_at: f.span,
                        }) {
                            errors.push(e);
                        }
                    }
                }
                Decl::StructDecl(s) => {
                    let qualified = format!("{}::{}", mod_name, s.name);
                    if let Err(e) = self.resolver.define(VarInfo {
                        name: qualified,
                        ty_name: "struct".to_string(),
                        mutable: false,
                        defined_at: s.span,
                    }) {
                        errors.push(e);
                    }
                }
                Decl::ModuleDecl(m) => {
                    let nested_name = format!("{}::{}", mod_name, m.name);
                    self.register_module_symbols(&nested_name, &m.decls, errors);
                }
                _ => {}
            }
        }
    }

    /// Check function bodies inside a module
    fn check_module_fns(&mut self, mod_name: &str, decls: &[Decl], errors: &mut Vec<SemaError>) {
        for decl in decls {
            match decl {
                Decl::FnDecl(f) => {
                    if let Err(mut errs) = self.check_fn(f) {
                        errors.append(&mut errs);
                    }
                }
                Decl::ModuleDecl(m) => {
                    let nested = format!("{}::{}", mod_name, m.name);
                    self.check_module_fns(&nested, &m.decls, errors);
                }
                _ => {}
            }
        }
    }

    /// Check a function declaration
    fn check_fn(&mut self, f: &FnDecl) -> Result<(), Vec<SemaError>> {
        let mut errors = Vec::new();
        self.resolver.push_scope();

        // Register parameters
        for (pname, pty) in &f.params {
            if let Err(e) = self.resolver.define(VarInfo {
                name: pname.clone(),
                ty_name: type_expr_to_name(pty),
                mutable: false,
                defined_at: pty.span(),
            }) {
                errors.push(e);
            }
        }

        // Set current return type
        self.current_ret_ty = f.ret_ty.as_ref().map(|t| resolve_type_name(&type_expr_to_name(t)));

        // Check body statements
        for stmt in &f.body.stmts {
            if let Err(e) = self.check_stmt(stmt) {
                errors.push(e);
            }
        }

        self.current_ret_ty = None;
        self.resolver.pop_scope();

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Check a single statement
    pub fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), SemaError> {
        match stmt {
            Stmt::Let { name, mutable, ty, init, span } => {
                let ty_name = match (ty, init) {
                    (Some(te), _) => type_expr_to_name(te),
                    (None, Some(expr)) => {
                        // For struct init, preserve the struct name as the type
                        if let Expr::StructInit { name: struct_name, .. } = expr {
                            let _ = self.check_expr(expr)?;
                            struct_name.clone()
                        } else {
                            let inferred = self.check_expr(expr)?;
                            hexa_type_name(&inferred)
                        }
                    }
                    (None, None) => "any".to_string(),
                };
                self.resolver.define(VarInfo {
                    name: name.clone(),
                    ty_name,
                    mutable: *mutable,
                    defined_at: *span,
                })?;
                Ok(())
            }
            Stmt::Assign { target, value, span } => {
                // Check the target is a valid lvalue and is mutable
                if let Expr::Ident(name, id_span) = target {
                    let info = self.resolver.lookup(name, *id_span)?;
                    if !info.mutable {
                        return Err(SemaError::OwnershipError {
                            span: *span,
                            message: format!("cannot assign to immutable binding '{}'", name),
                        });
                    }
                }
                let _ = self.check_expr(value)?;
                Ok(())
            }
            Stmt::Return { value, span } => {
                if let Some(expr) = value {
                    let ret_ty = self.check_expr(expr)?;
                    // Verify return type matches declared function return type
                    if let Some(ref expected) = self.current_ret_ty {
                        if !types_compatible(expected, &ret_ty) {
                            return Err(SemaError::TypeError {
                                span: *span,
                                expected: hexa_type_name(expected),
                                found: hexa_type_name(&ret_ty),
                            });
                        }
                    }
                }
                Ok(())
            }
            Stmt::If { cond, then_block, else_block, .. } => {
                let cond_ty = self.check_expr(cond)?;
                if !matches!(cond_ty, HexaType::Bool) {
                    return Err(SemaError::TypeError {
                        span: cond.span(),
                        expected: "bool".to_string(),
                        found: hexa_type_name(&cond_ty),
                    });
                }
                self.resolver.push_scope();
                for s in &then_block.stmts {
                    self.check_stmt(s)?;
                }
                self.resolver.pop_scope();
                if let Some(eb) = else_block {
                    self.resolver.push_scope();
                    for s in &eb.stmts {
                        self.check_stmt(s)?;
                    }
                    self.resolver.pop_scope();
                }
                Ok(())
            }
            Stmt::While { cond, body, .. } => {
                let cond_ty = self.check_expr(cond)?;
                if !matches!(cond_ty, HexaType::Bool) {
                    return Err(SemaError::TypeError {
                        span: cond.span(),
                        expected: "bool".to_string(),
                        found: hexa_type_name(&cond_ty),
                    });
                }
                self.resolver.push_scope();
                for s in &body.stmts {
                    self.check_stmt(s)?;
                }
                self.resolver.pop_scope();
                Ok(())
            }
            Stmt::ForLoop { var, iterable, body, span } => {
                let iter_ty = self.check_expr(iterable)?;
                let var_ty_name = match &iter_ty {
                    HexaType::Array(inner, _) => hexa_type_name(inner),
                    _ => "i64".to_string(),
                };
                self.resolver.push_scope();
                self.resolver.define(VarInfo {
                    name: var.clone(),
                    ty_name: var_ty_name,
                    mutable: false,
                    defined_at: *span,
                })?;
                for s in &body.stmts {
                    self.check_stmt(s)?;
                }
                self.resolver.pop_scope();
                Ok(())
            }
            Stmt::ExprStmt { expr, .. } => {
                let _ = self.check_expr(expr)?;
                Ok(())
            }
        }
    }

    /// Type-check an expression, returning its resolved HexaType
    pub fn check_expr(&mut self, expr: &Expr) -> Result<HexaType, SemaError> {
        match expr {
            Expr::IntLit(_, _) => Ok(HexaType::I64),
            Expr::FloatLit(_, _) => Ok(HexaType::F64),
            Expr::BoolLit(_, _) => Ok(HexaType::Bool),
            Expr::StrLit(_, _) => Ok(HexaType::Str),
            Expr::Ident(name, span) => {
                let info = self.resolver.lookup(name, *span)?;
                Ok(resolve_type_name(&info.ty_name))
            }
            Expr::Binary { op, lhs, rhs, span } => {
                let lt = self.check_expr(lhs)?;
                let rt = self.check_expr(rhs)?;
                match op {
                    BinOp::Add => {
                        if matches!(lt, HexaType::Str) && matches!(rt, HexaType::Str) {
                            Ok(HexaType::Str)
                        } else if lt.is_numeric() && rt.is_numeric() {
                            if matches!(lt, HexaType::F64) || matches!(rt, HexaType::F64) {
                                Ok(HexaType::F64)
                            } else {
                                Ok(HexaType::I64)
                            }
                        } else if matches!(lt, HexaType::Any) || matches!(rt, HexaType::Any) {
                            Ok(HexaType::Any)
                        } else {
                            Err(SemaError::TypeError {
                                span: *span,
                                expected: "numeric or str".to_string(),
                                found: format!("{:?} and {:?}", lt, rt),
                            })
                        }
                    }
                    BinOp::Sub | BinOp::Mul | BinOp::Div |
                    BinOp::Mod | BinOp::Pow => {
                        if lt.is_numeric() && rt.is_numeric() {
                            if matches!(lt, HexaType::F64) || matches!(rt, HexaType::F64) {
                                Ok(HexaType::F64)
                            } else {
                                Ok(HexaType::I64)
                            }
                        } else if matches!(lt, HexaType::Any) || matches!(rt, HexaType::Any) {
                            Ok(HexaType::Any)
                        } else {
                            Err(SemaError::TypeError {
                                span: *span,
                                expected: "numeric".to_string(),
                                found: format!("{:?} and {:?}", lt, rt),
                            })
                        }
                    }
                    BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt |
                    BinOp::Le | BinOp::Ge => Ok(HexaType::Bool),
                    BinOp::And | BinOp::Or => Ok(HexaType::Bool),
                    BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => Ok(HexaType::I64),
                    BinOp::Range | BinOp::RangeInclusive => Ok(HexaType::Any), // Range type TBD in Mk.II
                }
            }
            Expr::Unary { op, operand, span } => {
                let ot = self.check_expr(operand)?;
                match op {
                    UnOp::Neg => {
                        if ot.is_numeric() { Ok(ot) }
                        else {
                            Err(SemaError::TypeError {
                                span: *span,
                                expected: "numeric".to_string(),
                                found: hexa_type_name(&ot),
                            })
                        }
                    }
                    UnOp::Not => Ok(HexaType::Bool),
                }
            }
            Expr::Call { func, args, span } => {
                let func_ty = self.check_expr(func)?;
                for arg in args {
                    let _ = self.check_expr(arg)?;
                }
                // Resolve return type from function signature
                match &func_ty {
                    HexaType::Fn(_, ret) => Ok(*ret.clone()),
                    _ => {
                        // Try to resolve from function name in resolver
                        if let Expr::Ident(fname, fspan) = func.as_ref() {
                            if let Ok(info) = self.resolver.lookup(fname, *fspan) {
                                if let Some(ret_str) = info.ty_name.strip_prefix("fn -> ") {
                                    return Ok(resolve_type_name(ret_str));
                                }
                            }
                        }
                        Ok(HexaType::Any)
                    }
                }
            }
            Expr::Field { obj, name, span } => {
                let obj_ty = self.check_expr(obj)?;
                // Try to resolve struct field type
                // If obj is an Ident, look up its type name (which may be a struct name)
                let struct_name = match obj.as_ref() {
                    Expr::Ident(var_name, id_span) => {
                        self.resolver.lookup(var_name, *id_span).ok()
                            .map(|info| info.ty_name.clone())
                    }
                    _ => None,
                };
                if let Some(sname) = struct_name {
                    if let Some(fields) = self.struct_defs.get(&sname) {
                        if let Some((_, fty)) = fields.iter().find(|(fname, _)| fname == name) {
                            return Ok(fty.clone());
                        }
                    }
                }
                // Fallback: return Any for unresolved struct field access
                Ok(HexaType::Any)
            }
            Expr::Index { arr, idx, span } => {
                let arr_ty = self.check_expr(arr)?;
                let idx_ty = self.check_expr(idx)?;
                if !matches!(idx_ty, HexaType::I64) {
                    return Err(SemaError::TypeError {
                        span: *span,
                        expected: "i64".to_string(),
                        found: hexa_type_name(&idx_ty),
                    });
                }
                // If array type is known, return the element type
                match arr_ty {
                    HexaType::Array(inner, _) => Ok(*inner),
                    _ => Ok(HexaType::Any),
                }
            }
            Expr::StructInit { name, fields, span } => {
                // Verify struct exists
                let _ = self.resolver.lookup(name, *span)?;
                // Type-check each field value
                for (_, val) in fields {
                    let _ = self.check_expr(val)?;
                }
                // Return the struct type with field types
                if let Some(struct_fields) = self.struct_defs.get(name) {
                    let field_types: Vec<HexaType> = struct_fields.iter()
                        .map(|(_, ty)| ty.clone())
                        .collect();
                    Ok(HexaType::Struct(field_types))
                } else {
                    Ok(HexaType::Any)
                }
            }
            Expr::ArrayLit { elements, .. } => {
                // Type-check all elements; infer element type from first element
                let mut elem_ty = HexaType::Any;
                for elem in elements {
                    let t = self.check_expr(elem)?;
                    if matches!(elem_ty, HexaType::Any) {
                        elem_ty = t;
                    }
                }
                let count = elements.len();
                Ok(HexaType::Array(Box::new(elem_ty), count))
            }
            Expr::Match { scrutinee, arms, .. } => {
                // Check the scrutinee
                let _ = self.check_expr(scrutinee)?;
                // Check each arm body
                for arm in arms {
                    self.resolver.push_scope();
                    self.define_pattern_bindings(&arm.pattern)?;
                    if let crate::parser::ast::Pattern::Guard { condition, .. } = &arm.pattern {
                        let _ = self.check_expr(condition)?;
                    }
                    let _ = self.check_expr(&arm.body)?;
                    self.resolver.pop_scope();
                }
                // Match result type: Any for Mk.I (full inference in Mk.II)
                Ok(HexaType::Any)
            }
            Expr::TryExpr { expr, .. } => {
                let _ = self.check_expr(expr)?;
                Ok(HexaType::Any)
            }
            Expr::Closure { params, ret_ty, body, span } => {
                self.resolver.push_scope();
                let mut param_types = Vec::new();
                for (pname, pty) in params {
                    let ty_name = pty.as_ref()
                        .map(type_expr_to_name)
                        .unwrap_or_else(|| "any".to_string());
                    param_types.push(resolve_type_name(&ty_name));
                    let _ = self.resolver.define(VarInfo {
                        name: pname.clone(),
                        ty_name,
                        mutable: false,
                        defined_at: *span,
                    });
                }
                let body_ty = self.check_expr(body)?;
                self.resolver.pop_scope();
                let ret = ret_ty.as_ref()
                    .map(|t| resolve_type_name(&type_expr_to_name(t)))
                    .unwrap_or(body_ty);
                Ok(HexaType::Fn(param_types, Box::new(ret)))
            }
            Expr::GenericCall { func, args, .. } => {
                let _ = self.check_expr(func)?;
                for arg in args { let _ = self.check_expr(arg)?; }
                Ok(HexaType::Any)
            }
            Expr::Block(block) => {
                self.resolver.push_scope();
                for s in &block.stmts {
                    self.check_stmt(s)?;
                }
                self.resolver.pop_scope();
                Ok(HexaType::Void)
            }
        }
    }

    /// Recursively define pattern bindings in the current scope
    fn define_pattern_bindings(&mut self, pattern: &crate::parser::ast::Pattern) -> Result<(), SemaError> {
        use crate::parser::ast::Pattern;
        match pattern {
            Pattern::Wildcard(_) | Pattern::Literal(_, _) => Ok(()),
            Pattern::Binding(name, span) => {
                self.resolver.define(VarInfo {
                    name: name.clone(),
                    ty_name: "any".to_string(),
                    mutable: false,
                    defined_at: *span,
                })?;
                Ok(())
            }
            Pattern::Variant { bindings, .. } => {
                for sub in bindings { self.define_pattern_bindings(sub)?; }
                Ok(())
            }
            Pattern::StructDestructure { fields, span, .. } => {
                for (field_name, sub_pat) in fields {
                    if let Some(pat) = sub_pat {
                        self.define_pattern_bindings(pat)?;
                    } else {
                        self.resolver.define(VarInfo {
                            name: field_name.clone(),
                            ty_name: "any".to_string(),
                            mutable: false,
                            defined_at: *span,
                        })?;
                    }
                }
                Ok(())
            }
            Pattern::Tuple { elements, .. } => {
                for elem in elements { self.define_pattern_bindings(elem)?; }
                Ok(())
            }
            Pattern::Guard { pattern: inner, .. } => {
                self.define_pattern_bindings(inner)
            }
        }
    }

}

// ── Helpers ──

/// Convert a TypeExpr to a simple string name
fn type_expr_to_name(te: &TypeExpr) -> String {
    match te {
        TypeExpr::Named(name, _) => name.clone(),
        TypeExpr::Array(inner, size, _) => format!("[{}; {}]", type_expr_to_name(inner), size),
        TypeExpr::Fn(params, ret, _) => {
            let ps: Vec<String> = params.iter().map(type_expr_to_name).collect();
            format!("fn({}) -> {}", ps.join(", "), type_expr_to_name(ret))
        }
        TypeExpr::TypeParam(name, _) => name.clone(),
    }
}

/// Map a type name string to a HexaType
fn resolve_type_name(name: &str) -> HexaType {
    match name {
        "i64" => HexaType::I64,
        "f64" => HexaType::F64,
        "bool" => HexaType::Bool,
        "char" => HexaType::Char,
        "str" => HexaType::Str,
        "byte" => HexaType::Byte,
        "void" => HexaType::Void,
        "any" => HexaType::Any,
        _ => HexaType::Any, // User-defined types resolve to Any in Mk.I
    }
}

/// Check if two types are compatible (for assignment / return checking).
/// Any is compatible with everything (dynamic typing fallback for Mk.I).
fn types_compatible(expected: &HexaType, found: &HexaType) -> bool {
    if matches!(expected, HexaType::Any) || matches!(found, HexaType::Any) {
        return true;
    }
    match (expected, found) {
        (HexaType::I64, HexaType::I64) => true,
        (HexaType::F64, HexaType::F64) => true,
        (HexaType::Bool, HexaType::Bool) => true,
        (HexaType::Char, HexaType::Char) => true,
        (HexaType::Str, HexaType::Str) => true,
        (HexaType::Byte, HexaType::Byte) => true,
        (HexaType::Void, HexaType::Void) => true,
        // Numeric coercion: i64 and f64 are compatible
        (HexaType::I64, HexaType::F64) | (HexaType::F64, HexaType::I64) => true,
        // Struct/Enum/Array/Fn: structural check deferred to Mk.II
        (HexaType::Struct(_), HexaType::Struct(_)) => true,
        (HexaType::Enum(_), HexaType::Enum(_)) => true,
        (HexaType::Array(a, _), HexaType::Array(b, _)) => types_compatible(a, b),
        // Trait objects are compatible with same trait
        (HexaType::TraitObject(a), HexaType::TraitObject(b)) => a == b,
        // Closure objects: compatible if param/return types match
        (HexaType::ClosureObj(_, _), HexaType::ClosureObj(_, _)) => true,
        // Fn and ClosureObj are compatible (closure implements Fn)
        (HexaType::Fn(_, _), HexaType::ClosureObj(_, _)) => true,
        (HexaType::ClosureObj(_, _), HexaType::Fn(_, _)) => true,
        _ => false,
    }
}

/// Get a display name for a HexaType
fn hexa_type_name(ty: &HexaType) -> String {
    match ty {
        HexaType::I64 => "i64".to_string(),
        HexaType::F64 => "f64".to_string(),
        HexaType::Bool => "bool".to_string(),
        HexaType::Char => "char".to_string(),
        HexaType::Str => "str".to_string(),
        HexaType::Byte => "byte".to_string(),
        HexaType::Void => "void".to_string(),
        HexaType::Any => "any".to_string(),
        HexaType::Struct(fields) => format!("struct({} fields)", fields.len()),
        HexaType::Enum(variants) => format!("enum({} variants)", variants.len()),
        HexaType::Array(inner, size) => format!("[{}; {}]", hexa_type_name(inner), size),
        HexaType::Fn(params, ret) => {
            let ps: Vec<String> = params.iter().map(hexa_type_name).collect();
            format!("fn({}) -> {}", ps.join(", "), hexa_type_name(ret))
        }
        HexaType::TraitObject(name) => format!("dyn {}", name),
        HexaType::ClosureObj(params, ret) => {
            let ps: Vec<String> = params.iter().map(hexa_type_name).collect();
            format!("closure({}) -> {}", ps.join(", "), hexa_type_name(ret))
        }
    }
}
