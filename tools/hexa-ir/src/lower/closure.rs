/// Closure lambda lifting — Mk.II: full closure conversion
///
/// Closures are compiled via lambda lifting:
///   1. Analyze free variables in the closure body
///   2. Create a capture struct (env) with captured variables
///   3. Lift the closure body to a top-level function (__closure_N)
///      with an extra first parameter for the env struct
///   4. At the call site, allocate env, store captures, return (fn_ptr, env_ptr)
///   5. Closure invocation loads fn_ptr + passes env_ptr as first arg
///
/// n=6 alignment: ClosureObj = (fn_ptr, env_ptr) = φ·σ-τ = 16 bytes

use crate::ir::*;
use crate::parser::ast;

/// Information about a captured variable
#[derive(Clone, Debug)]
pub struct CaptureInfo {
    /// Name of the captured variable
    pub name: String,
    /// SSA register in the enclosing scope
    pub outer_reg: usize,
    /// Offset within the capture struct (bytes)
    pub capture_offset: usize,
    /// Whether captured by reference (true) or by value (false)
    pub by_ref: bool,
}

/// Closure representation in IR
#[derive(Clone, Debug)]
pub struct ClosureIR {
    /// The captures this closure needs
    pub captures: Vec<CaptureInfo>,
    /// Register holding the allocated capture struct
    pub env_reg: usize,
    /// The function that implements the closure body
    pub body_fn: String,
}

/// Analyze free variables in an expression — returns names that are
/// referenced but not defined within the expression itself
pub fn analyze_free_vars(expr: &ast::Expr, bound: &[String]) -> Vec<String> {
    let mut free = Vec::new();
    match expr {
        ast::Expr::Ident(name, _) => {
            if !bound.contains(name) {
                free.push(name.clone());
            }
        }
        ast::Expr::Binary { lhs, rhs, .. } => {
            free.extend(analyze_free_vars(lhs, bound));
            free.extend(analyze_free_vars(rhs, bound));
        }
        ast::Expr::Unary { operand, .. } => {
            free.extend(analyze_free_vars(operand, bound));
        }
        ast::Expr::Call { func, args, .. } => {
            free.extend(analyze_free_vars(func, bound));
            for arg in args {
                free.extend(analyze_free_vars(arg, bound));
            }
        }
        ast::Expr::Field { obj, .. } => {
            free.extend(analyze_free_vars(obj, bound));
        }
        ast::Expr::Index { arr, idx, .. } => {
            free.extend(analyze_free_vars(arr, bound));
            free.extend(analyze_free_vars(idx, bound));
        }
        ast::Expr::Block(block) => {
            let mut inner_bound = bound.to_vec();
            for stmt in &block.stmts {
                if let ast::Stmt::Let { name, init, .. } = stmt {
                    if let Some(init_expr) = init {
                        free.extend(analyze_free_vars(init_expr, &inner_bound));
                    }
                    inner_bound.push(name.clone());
                }
            }
        }
        ast::Expr::Closure { params, body, .. } => {
            let mut inner_bound = bound.to_vec();
            for (pname, _) in params {
                inner_bound.push(pname.clone());
            }
            free.extend(analyze_free_vars(body, &inner_bound));
        }
        ast::Expr::Match { scrutinee, arms, .. } => {
            free.extend(analyze_free_vars(scrutinee, bound));
            for arm in arms {
                free.extend(analyze_free_vars(&arm.body, bound));
            }
        }
        ast::Expr::TryExpr { expr, .. } => {
            free.extend(analyze_free_vars(expr, bound));
        }
        ast::Expr::GenericCall { func, args, .. } => {
            free.extend(analyze_free_vars(func, bound));
            for arg in args {
                free.extend(analyze_free_vars(arg, bound));
            }
        }
        _ => {} // literals have no free variables
    }
    // Deduplicate
    free.sort();
    free.dedup();
    free
}

/// Lift a closure body to a top-level HexaFunction.
///
/// The lifted function has signature:
///   fn __closure_N(env: Any, param0: T0, param1: T1, ...) -> RetTy { body }
///
/// Inside the body, captured variables are loaded from the env struct:
///   %cap_0 = Load env[0]
///   %cap_1 = Load env[8]
///   ...
pub fn lift_closure_to_function(
    closure_name: &str,
    params: &[(String, Option<ast::TypeExpr>)],
    body: &ast::Expr,
    captures: &[CaptureInfo],
    ctx: &mut super::LowerContext,
) -> HexaFunction {
    // Save and reset per-function state
    let saved_vars = ctx.vars.clone();
    let saved_var_types = ctx.var_struct_types.clone();
    let saved_string_pool = ctx.string_pool.clone();
    let saved_reg = ctx.reg_counter;
    let saved_block = ctx.block_counter;
    ctx.reg_counter = 0;
    ctx.block_counter = 0;
    ctx.vars.clear();
    ctx.var_struct_types.clear();
    ctx.string_pool.clear();

    // First parameter: env struct pointer
    let env_reg = ctx.fresh_reg(); // reg 0
    ctx.bind_var("__env", env_reg);

    // Bind closure parameters (regs 1..N)
    let mut fn_params = vec![("__env".to_string(), HexaType::Any)];
    for (pname, pty) in params {
        let reg = ctx.fresh_reg();
        ctx.bind_var(pname, reg);
        let ty = pty.as_ref()
            .map(|t| super::lower_type_expr(t))
            .unwrap_or(HexaType::Any);
        fn_params.push((pname.clone(), ty));
    }

    // Create entry block
    let entry_id = ctx.fresh_block();
    let mut entry_block = HexaBlock {
        id: entry_id,
        instrs: Vec::new(),
        successors: Vec::new(),
    };

    // Load captured variables from env struct
    for (i, cap) in captures.iter().enumerate() {
        if i == 0 {
            // First capture: direct load from env pointer
            let cap_reg = ctx.fresh_reg();
            entry_block.instrs.push(HexaInstr {
                op: HexaOp::Load,
                dest: Some(cap_reg),
                args: vec![env_reg],
                ty: HexaType::I64,
                label: Some(format!("cap_{}", cap.name)),
            });
            ctx.bind_var(&cap.name, cap_reg);
        } else {
            // Subsequent captures: env_ptr + offset, then load
            let offset_reg = ctx.fresh_reg();
            entry_block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(offset_reg),
                args: vec![i * 8],
                ty: HexaType::I64,
                label: None,
            });
            let addr_reg = ctx.fresh_reg();
            entry_block.instrs.push(HexaInstr {
                op: HexaOp::Add,
                dest: Some(addr_reg),
                args: vec![env_reg, offset_reg],
                ty: HexaType::I64,
                label: None,
            });
            let cap_reg = ctx.fresh_reg();
            entry_block.instrs.push(HexaInstr {
                op: HexaOp::Load,
                dest: Some(cap_reg),
                args: vec![addr_reg],
                ty: HexaType::I64,
                label: Some(format!("cap_{}", cap.name)),
            });
            ctx.bind_var(&cap.name, cap_reg);
        }
    }

    // Lower the closure body expression
    let mut extra_blocks = Vec::new();
    let result_reg = super::expr_lower::lower_expr(ctx, &mut entry_block, body);

    // Emit return with the body result
    entry_block.instrs.push(HexaInstr {
        op: HexaOp::Return,
        dest: None,
        args: vec![result_reg],
        ty: HexaType::Void,
        label: None,
    });

    // Drain pending blocks
    let pending = std::mem::take(&mut ctx.pending_blocks);
    extra_blocks.extend(pending);

    extra_blocks.push(entry_block);
    extra_blocks.sort_by_key(|b| b.id);

    let func = HexaFunction {
        name: closure_name.to_string(),
        blocks: extra_blocks,
        params: fn_params,
        ret_ty: HexaType::Any,
        string_pool: ctx.string_pool.clone(),
    };

    // Restore context state
    ctx.vars = saved_vars;
    ctx.var_struct_types = saved_var_types;
    ctx.string_pool = saved_string_pool;
    ctx.reg_counter = saved_reg;
    ctx.block_counter = saved_block;

    func
}
