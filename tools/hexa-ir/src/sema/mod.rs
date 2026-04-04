/// HEXA-LANG Semantic Analysis — n=6 files
///
/// Three analysis layers:
///   Layer 1: Name resolution + type checking (Hindley-Milner Mk.I)
///   Layer 2: Ownership analysis (single-owner, no use-after-move)
///   Layer 3: Trait resolution (Mk.II placeholder)
///
/// File count = n = 6: mod.rs, resolve.rs, typecheck.rs, ownership.rs, trait_impl.rs, error.rs

pub mod resolve;
pub mod typecheck;
pub mod ownership;
pub mod trait_impl;
pub mod error;

pub use error::SemaError;

use crate::parser::ast::{Program, TypeExpr};
use typecheck::TypeChecker;
use ownership::OwnershipChecker;
use trait_impl::TraitResolver;

/// Convert a TypeExpr to a simple string name (duplicated from typecheck for mod-level use)
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

/// Check if a type name represents a Copy type
fn is_copy_type_name(name: &str) -> bool {
    matches!(name, "i64" | "f64" | "bool" | "char" | "byte" | "void" | "any")
}

/// Run all semantic analysis passes on a parsed program.
/// Returns Ok(()) if the program is semantically valid,
/// or Err(errors) with all detected issues.
pub fn analyze(program: &Program) -> Result<(), Vec<SemaError>> {
    let mut all_errors = Vec::new();

    // Layer 1: Type checking (includes name resolution)
    let mut tc = TypeChecker::new();
    if let Err(mut errs) = tc.check_program(program) {
        all_errors.append(&mut errs);
    }

    // Layer 2: Ownership analysis per function
    for decl in &program.decls {
        match decl {
            crate::parser::ast::Decl::FnDecl(f) => {
                let mut oc = OwnershipChecker::new();
                for (pname, pty) in &f.params {
                    let ty_name = type_expr_to_name(pty);
                    let is_copy = is_copy_type_name(&ty_name);
                    oc.define_with_info(pname, f.span, false, is_copy);
                }
                let mut errs = oc.check_block(&f.body);
                all_errors.append(&mut errs);
            }
            crate::parser::ast::Decl::ModuleDecl(m) => {
                check_module_ownership(&m.decls, &mut all_errors);
            }
            _ => {}
        }
    }

    // Layer 3: Trait resolution (Mk.II — currently no-op)
    let mut tr = TraitResolver::new();
    if let Err(mut errs) = tr.resolve_traits(program) {
        all_errors.append(&mut errs);
    }

    if all_errors.is_empty() {
        Ok(())
    } else {
        Err(all_errors)
    }
}

/// Check ownership for functions inside modules (recursive)
fn check_module_ownership(decls: &[crate::parser::ast::Decl], errors: &mut Vec<SemaError>) {
    for decl in decls {
        match decl {
            crate::parser::ast::Decl::FnDecl(f) => {
                let mut oc = OwnershipChecker::new();
                for (pname, pty) in &f.params {
                    let ty_name = type_expr_to_name(pty);
                    let is_copy = is_copy_type_name(&ty_name);
                    oc.define_with_info(pname, f.span, false, is_copy);
                }
                let mut errs = oc.check_block(&f.body);
                errors.append(&mut errs);
            }
            crate::parser::ast::Decl::ModuleDecl(m) => {
                check_module_ownership(&m.decls, errors);
            }
            _ => {}
        }
    }
}
