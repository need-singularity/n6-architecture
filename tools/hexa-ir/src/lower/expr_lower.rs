/// Expression lowering — AST Expr -> SSA register
///
/// Each expression emits instructions into the current block
/// and returns the SSA register holding the result.

use crate::ir::*;
use crate::parser::ast::{Expr, BinOp, UnOp};
use super::LowerContext;

/// Lower an expression, returning the SSA register holding the result
pub fn lower_expr(ctx: &mut LowerContext, block: &mut HexaBlock, expr: &Expr) -> usize {
    match expr {
        Expr::IntLit(v, _) => {
            // Emit: %r = Alloc : I64  (conceptual "materialize constant")
            // In a real SSA, constants are implicit. We use Alloc+Store for addressable values,
            // but for pure integer literals we just emit a Copy from a virtual const register.
            // For Mk.I: allocate a register, the value is tracked by the register number.
            let dest = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(dest),
                args: vec![*v as usize], // encode the constant value in args[0]
                ty: HexaType::I64,
            label: None,
            });
            dest
        }

        Expr::FloatLit(_v, _) => {
            let dest = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(dest),
                args: vec![],
                ty: HexaType::F64,
            label: None,
            });
            dest
        }

        Expr::BoolLit(v, _) => {
            let dest = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(dest),
                args: vec![if *v { 1 } else { 0 }],
                ty: HexaType::Bool,
            label: None,
            });
            dest
        }

        Expr::StrLit(s, _) => {
            // Intern string in pool; emit Alloc with pool_index and length
            let pool_idx = ctx.intern_string(s);
            let str_len = s.len();
            let dest = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(dest),
                args: vec![pool_idx, str_len],
                ty: HexaType::Str,
                label: Some(format!("str_pool_{}", pool_idx)),
            });
            dest
        }

        Expr::Ident(name, _) => {
            // Check if this is an enum variant: "EnumName::VariantName"
            if let Some((tag, _payload)) = ctx.lookup_enum_variant(name) {
                // Emit the tag as an integer constant
                let dest = ctx.fresh_reg();
                block.instrs.push(HexaInstr {
                    op: HexaOp::Alloc,
                    dest: Some(dest),
                    args: vec![tag],
                    ty: HexaType::I64,
                    label: Some(format!("enum_tag_{}", name)),
                });
                return dest;
            }
            // Check if this is a module-qualified path: "mod::func" -> mangled "mod__func"
            let resolved_name = if name.contains("::") && ctx.lookup_enum_variant(name).is_none() {
                name.replace("::", "__")
            } else {
                name.clone()
            };
            // Look up the variable's SSA register and emit a Load
            let src = ctx.lookup_var(&resolved_name)
                .or_else(|| ctx.lookup_var(name))
                .unwrap_or(0);
            let dest = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Load,
                dest: Some(dest),
                args: vec![src],
                ty: HexaType::Any, // type resolved by pass_type_inference
                label: Some(resolved_name), // carry mangled name for codegen
            });
            dest
        }

        Expr::Binary { op, lhs, rhs, .. } => {
            let l = lower_expr(ctx, block, lhs);
            let r = lower_expr(ctx, block, rhs);
            let dest = ctx.fresh_reg();
            let (hexa_op, ty) = lower_binop(op);
            let cmp_label = cmp_label(op);
            block.instrs.push(HexaInstr {
                op: hexa_op,
                dest: Some(dest),
                args: vec![l, r],
                ty, label: cmp_label,
            });
            dest
        }

        Expr::Unary { op, operand, .. } => {
            let src = lower_expr(ctx, block, operand);
            let dest = ctx.fresh_reg();
            let hexa_op = match op {
                UnOp::Neg => HexaOp::Neg,
                UnOp::Not => HexaOp::Neg, // Mk.I: logical not as arithmetic neg
            };
            block.instrs.push(HexaInstr {
                op: hexa_op,
                dest: Some(dest),
                args: vec![src],
                ty: HexaType::Any,
            label: None,
            });
            dest
        }

        Expr::Call { func, args, .. } => {
            // Check for method call: obj.method(args) -> TypeName__method(obj, args)
            if let Expr::Field { obj, name: method_name, .. } = func.as_ref() {
                let struct_type_name = match obj.as_ref() {
                    Expr::Ident(var_name, _) => ctx.lookup_var_struct_type(var_name).cloned(),
                    _ => None,
                };
                if let Some(type_name) = struct_type_name {
                    let dispatch_key = (type_name.clone(), method_name.clone());
                    if let Some(mangled) = ctx.method_dispatch.get(&dispatch_key).cloned() {
                        let self_reg = match obj.as_ref() {
                            Expr::Ident(var_name, _) => ctx.lookup_var(var_name).unwrap_or(0),
                            _ => lower_expr(ctx, block, obj),
                        };
                        let fn_name_reg = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Load,
                            dest: Some(fn_name_reg),
                            args: vec![0],
                            ty: HexaType::Any,
                            label: Some(mangled.clone()),
                        });
                        let mut arg_regs = vec![fn_name_reg, self_reg];
                        for arg in args {
                            let r = lower_expr(ctx, block, arg);
                            arg_regs.push(r);
                        }
                        let dest = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Call,
                            dest: Some(dest),
                            args: arg_regs,
                            ty: HexaType::Any,
                            label: Some(mangled),
                        });
                        return dest;
                    }
                }
            }

            // Regular function call
            // Mangle module paths: "mod::func" -> "mod__func"
            let call_name = match func.as_ref() {
                Expr::Ident(name, _) => {
                    if name.contains("::") && ctx.lookup_enum_variant(name).is_none() {
                        Some(name.replace("::", "__"))
                    } else {
                        Some(name.clone())
                    }
                }
                _ => None,
            };
            // Lower the callee expression
            let func_reg = lower_expr(ctx, block, func);
            // Lower each argument
            let mut arg_regs = vec![func_reg];
            for arg in args {
                let r = lower_expr(ctx, block, arg);
                arg_regs.push(r);
            }
            let dest = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Call,
                dest: Some(dest),
                args: arg_regs,
                ty: HexaType::Any, // return type resolved later
                label: call_name, // carry function name for codegen
            });
            dest
        }

        Expr::Field { obj, name, .. } => {
            // Resolve the struct type from the object expression
            // For Ident objects, look up var -> struct type mapping
            let struct_name = match obj.as_ref() {
                Expr::Ident(var_name, _) => ctx.lookup_var_struct_type(var_name).cloned(),
                _ => None,
            };

            // Get the base register — for struct variables, get base directly (no Load)
            let obj_reg = match obj.as_ref() {
                Expr::Ident(var_name, _) => {
                    ctx.lookup_var(var_name).unwrap_or(0)
                }
                _ => lower_expr(ctx, block, obj),
            };

            if let Some(sname) = struct_name {
                if let Some((_idx, byte_offset, field_ty)) = ctx.lookup_struct_field(&sname, name) {
                    if byte_offset == 0 {
                        // First field: load directly from base
                        let dest = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Load,
                            dest: Some(dest),
                            args: vec![obj_reg],
                            ty: field_ty,
                            label: Some(format!("field_{}", name)),
                        });
                        dest
                    } else {
                        // Compute address: base + byte_offset
                        let offset_reg = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Alloc,
                            dest: Some(offset_reg),
                            args: vec![byte_offset],
                            ty: HexaType::I64,
                            label: None,
                        });
                        let addr = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Add,
                            dest: Some(addr),
                            args: vec![obj_reg, offset_reg],
                            ty: HexaType::I64,
                            label: None,
                        });
                        let dest = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Load,
                            dest: Some(dest),
                            args: vec![addr],
                            ty: field_ty,
                            label: Some(format!("field_{}", name)),
                        });
                        dest
                    }
                } else {
                    // Field not found — fallback to simple Load
                    let dest = ctx.fresh_reg();
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Load,
                        dest: Some(dest),
                        args: vec![obj_reg],
                        ty: HexaType::Any,
                        label: None,
                    });
                    dest
                }
            } else {
                // No struct type info — fallback to simple Load
                let dest = ctx.fresh_reg();
                block.instrs.push(HexaInstr {
                    op: HexaOp::Load,
                    dest: Some(dest),
                    args: vec![obj_reg],
                    ty: HexaType::Any,
                    label: None,
                });
                dest
            }
        }

        Expr::Index { arr, idx, .. } => {
            // For array indexing, get the base address directly (skip Load indirection)
            let arr_reg = match arr.as_ref() {
                Expr::Ident(name, _) => {
                    // Direct lookup: return the array base register without emitting Load
                    ctx.lookup_var(name).unwrap_or(0)
                }
                _ => lower_expr(ctx, block, arr),
            };
            let idx_reg = lower_expr(ctx, block, idx);
            // Proof obligation: assert index >= 0 (zero runtime cost)
            // Emit ProofAssert with idx_reg; verifier checks idx < array_length
            super::proof_emit::emit_proof_assert(ctx, block, idx_reg);
            // Compute element offset: idx * element_size (σ-τ=8 bytes for i64)
            let elem_size = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(elem_size),
                args: vec![8], // i64 element size = σ-τ=8 bytes
                ty: HexaType::I64,
                label: None,
            });
            let offset = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Mul,
                dest: Some(offset),
                args: vec![idx_reg, elem_size],
                ty: HexaType::I64,
                label: None,
            });
            // Compute address: base + offset
            let addr = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Add,
                dest: Some(addr),
                args: vec![arr_reg, offset],
                ty: HexaType::I64,
                label: None,
            });
            let dest = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Load,
                dest: Some(dest),
                args: vec![addr],
                ty: HexaType::Any,
                label: None,
            });
            dest
        }

        Expr::StructInit { name: struct_name, fields, .. } => {
            // Look up struct definition for proper byte-offset layout
            let struct_fields = ctx.struct_defs.get(struct_name).cloned();
            let total_size = ctx.struct_size(struct_name);
            let alloc_size = if total_size > 0 { total_size } else { fields.len() * 8 };

            // Build field type list for HexaType::Struct
            let field_types: Vec<HexaType> = struct_fields.as_ref()
                .map(|fs| fs.iter().map(|(_, ty)| ty.clone()).collect())
                .unwrap_or_else(|| vec![HexaType::Any; fields.len()]);

            // Allocate space for the entire struct
            let base = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(base),
                args: vec![alloc_size],
                ty: HexaType::Struct(field_types),
                label: Some(format!("struct_{}", struct_name)),
            });

            // Store each field at its proper byte offset
            for (field_name, val_expr) in fields.iter() {
                let val_reg = lower_expr(ctx, block, val_expr);

                // Look up byte offset from struct definition
                let byte_offset = struct_fields.as_ref()
                    .and_then(|fs| {
                        let mut off = 0usize;
                        for (fname, fty) in fs.iter() {
                            if fname == field_name {
                                return Some(off);
                            }
                            off += fty.size_bytes();
                        }
                        None
                    })
                    .unwrap_or(0);

                if byte_offset == 0 {
                    // First field: store directly at base
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Store,
                        dest: None,
                        args: vec![base, val_reg],
                        ty: HexaType::Void,
                        label: None,
                    });
                } else {
                    let offset_reg = ctx.fresh_reg();
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Alloc,
                        dest: Some(offset_reg),
                        args: vec![byte_offset],
                        ty: HexaType::I64,
                        label: None,
                    });
                    let field_addr = ctx.fresh_reg();
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Add,
                        dest: Some(field_addr),
                        args: vec![base, offset_reg],
                        ty: HexaType::I64,
                        label: None,
                    });
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Store,
                        dest: None,
                        args: vec![field_addr, val_reg],
                        ty: HexaType::Void,
                        label: None,
                    });
                }
            }
            base
        }

        Expr::ArrayLit { elements, .. } => {
            // Allocate contiguous space: element_count * element_size (σ-τ=8 bytes)
            let elem_count = elements.len();
            let total_size = elem_count * 8; // i64 = σ-τ=8 bytes each
            let base = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(base),
                args: vec![total_size],
                ty: HexaType::Array(Box::new(HexaType::I64), elem_count),
                label: None,
            });
            // Store each element at base + i * 8
            // First element at offset 0 stores directly to base
            for (i, elem) in elements.iter().enumerate() {
                let val_reg = lower_expr(ctx, block, elem);
                if i == 0 {
                    // Store directly at base address (offset 0)
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Store,
                        dest: None,
                        args: vec![base, val_reg],
                        ty: HexaType::Void,
                        label: None,
                    });
                } else {
                    let byte_offset = i * 8;
                    let offset_reg = ctx.fresh_reg();
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Alloc,
                        dest: Some(offset_reg),
                        args: vec![byte_offset],
                        ty: HexaType::I64,
                        label: None,
                    });
                    let elem_addr = ctx.fresh_reg();
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Add,
                        dest: Some(elem_addr),
                        args: vec![base, offset_reg],
                        ty: HexaType::I64,
                        label: None,
                    });
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Store,
                        dest: None,
                        args: vec![elem_addr, val_reg],
                        ty: HexaType::Void,
                        label: None,
                    });
                }
            }
            base
        }

        Expr::Match { scrutinee, arms, .. } => {
            return lower_match_expr(ctx, block, scrutinee, arms);
        }

        Expr::TryExpr { expr, .. } => {
            return lower_try_expr(ctx, block, expr);
        }

        Expr::Closure { params, body, .. } => {
            // Closure lowering: allocate env struct, store captures, return env_reg
            let param_names: Vec<&str> = params.iter().map(|(n, _)| n.as_str()).collect();
            let captured: Vec<(String, usize)> = ctx.vars.iter()
                .filter(|(name, _)| !param_names.contains(&name.as_str()))
                .map(|(name, reg)| (name.clone(), *reg))
                .collect();

            let env_size = if captured.is_empty() { 8 } else { captured.len() * 8 };
            let env_reg = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(env_reg),
                args: vec![env_size],
                ty: HexaType::Any,
                label: Some("closure_env".to_string()),
            });

            // Store captured variables into environment
            for (i, (_cap_name, cap_reg)) in captured.iter().enumerate() {
                if i == 0 {
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Store, dest: None,
                        args: vec![env_reg, *cap_reg],
                        ty: HexaType::Void, label: None,
                    });
                } else {
                    let offset_reg = ctx.fresh_reg();
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Alloc, dest: Some(offset_reg),
                        args: vec![i * 8], ty: HexaType::I64, label: None,
                    });
                    let addr = ctx.fresh_reg();
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Add, dest: Some(addr),
                        args: vec![env_reg, offset_reg],
                        ty: HexaType::I64, label: None,
                    });
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Store, dest: None,
                        args: vec![addr, *cap_reg],
                        ty: HexaType::Void, label: None,
                    });
                }
            }

            let closure_id = ctx.next_closure_id();
            let closure_fn_name = format!("__closure_{}", closure_id);

            // Record closure for lambda lifting
            ctx.closure_fns.push(super::closure::ClosureIR {
                captures: captured.iter().enumerate().map(|(i, (name, reg))| {
                    super::closure::CaptureInfo {
                        name: name.clone(), outer_reg: *reg,
                        capture_offset: i * 8, by_ref: false,
                    }
                }).collect(),
                env_reg,
                body_fn: closure_fn_name.clone(),
            });

            let result = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc, dest: Some(result),
                args: vec![env_reg],
                ty: HexaType::Fn(vec![], Box::new(HexaType::Any)),
                label: Some(closure_fn_name),
            });
            result
        }

        Expr::GenericCall { func, args, .. } => {
            // Generic calls are monomorphized — lower as regular Call
            let call_name = match func.as_ref() {
                Expr::Ident(name, _) => Some(name.clone()),
                _ => None,
            };
            let func_reg = lower_expr(ctx, block, func);
            let mut arg_regs = vec![func_reg];
            for arg in args {
                arg_regs.push(lower_expr(ctx, block, arg));
            }
            let dest = ctx.fresh_reg();
            block.instrs.push(HexaInstr {
                op: HexaOp::Call, dest: Some(dest),
                args: arg_regs, ty: HexaType::Any,
                label: call_name,
            });
            dest
        }

        Expr::Block(blk) => {
            // Lower statements in the block; result is last expression if any
            let mut last_reg = ctx.fresh_reg();
            // Emit a void placeholder
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(last_reg),
                args: vec![],
                ty: HexaType::Void,
            label: None,
            });
            for stmt in &blk.stmts {
                let r = super::stmt_lower::lower_stmt(ctx, block, stmt);
                last_reg = r;
            }
            last_reg
        }
    }
}

/// Lower a match expression into multi-block Switch-based control flow.
///
/// Strategy:
///   1. Evaluate the scrutinee (produces tag value for enums)
///   2. For each arm, create a separate block
///   3. Emit cascading compare-and-branch from the entry block
///   4. Each arm block lowers its body, then jumps to merge block
///   5. The merge block receives the result via Load from shared slot
fn lower_match_expr(
    ctx: &mut LowerContext,
    block: &mut HexaBlock,
    scrutinee: &Expr,
    arms: &[crate::parser::ast::MatchArm],
) -> usize {
    use crate::parser::ast::Pattern;

    let scrutinee_reg = lower_expr(ctx, block, scrutinee);

    let merge_id = ctx.fresh_block();

    // Alloc a result slot that all arms will Store into
    let result_reg = ctx.fresh_reg();
    block.instrs.push(HexaInstr {
        op: HexaOp::Alloc,
        dest: Some(result_reg),
        args: vec![0],
        ty: HexaType::I64,
        label: Some("match_result".to_string()),
    });

    // Collect arm block IDs and tags
    let mut arm_infos: Vec<(usize, Option<usize>)> = Vec::new();
    let mut wildcard_id: Option<usize> = None;

    for arm in arms {
        let arm_id = ctx.fresh_block();
        let tag = extract_pattern_tag(ctx, &arm.pattern);
        if tag.is_none() && wildcard_id.is_none() {
            wildcard_id = Some(arm_id);
        }
        arm_infos.push((arm_id, tag));
    }

    let default_target = wildcard_id.unwrap_or(merge_id);

    // Build a chain of compare-and-branch blocks.
    // We swap `block` to new check blocks as we go.
    let non_wildcard: Vec<(usize, usize)> = arm_infos.iter()
        .filter_map(|(id, tag)| tag.map(|t| (*id, t)))
        .collect();

    for (i, (arm_id, tag_val)) in non_wildcard.iter().enumerate() {
        let tag_reg = ctx.fresh_reg();
        block.instrs.push(HexaInstr {
            op: HexaOp::Alloc,
            dest: Some(tag_reg),
            args: vec![*tag_val],
            ty: HexaType::I64,
            label: None,
        });

        let cmp_reg = ctx.fresh_reg();
        block.instrs.push(HexaInstr {
            op: HexaOp::Sub,
            dest: Some(cmp_reg),
            args: vec![scrutinee_reg, tag_reg],
            ty: HexaType::Bool,
            label: Some("eq".to_string()),
        });

        // Determine else target: next check block or default
        let is_last = i + 1 >= non_wildcard.len();
        let else_target = if is_last { default_target } else { ctx.fresh_block() };

        // Branch: cmp != 0 → else_target (no match), cmp == 0 → arm_id (match)
        block.instrs.push(HexaInstr {
            op: HexaOp::Branch,
            dest: None,
            args: vec![cmp_reg, else_target, *arm_id],
            ty: HexaType::Void,
            label: None,
        });
        block.successors.push(*arm_id);
        block.successors.push(else_target);

        if !is_last {
            // Swap current block → push old, continue in else_target
            let prev_blk = HexaBlock {
                id: block.id,
                instrs: std::mem::take(&mut block.instrs),
                successors: std::mem::take(&mut block.successors),
            };
            ctx.pending_blocks.push(prev_blk);
            block.id = else_target;
        }
    }

    // If the last check block doesn't fall through to default naturally,
    // and there's no wildcard but we need to reach merge, add a Jump
    if non_wildcard.is_empty() {
        // All arms are wildcards — just jump to the first arm
        if let Some(wid) = wildcard_id {
            block.instrs.push(HexaInstr {
                op: HexaOp::Jump,
                dest: None,
                args: vec![wid],
                ty: HexaType::Void,
                label: None,
            });
            block.successors.push(wid);
        }
    }

    // Lower each arm body into its own block
    for (i, arm) in arms.iter().enumerate() {
        let arm_id = arm_infos[i].0;
        let mut arm_block = HexaBlock {
            id: arm_id,
            instrs: Vec::new(),
            successors: Vec::new(),
        };

        // Emit pattern bindings (struct destructure, variable bindings, etc.)
        emit_pattern_bindings(ctx, &mut arm_block, &arm.pattern, scrutinee_reg);

        let arm_val = lower_expr(ctx, &mut arm_block, &arm.body);

        // Store result
        arm_block.instrs.push(HexaInstr {
            op: HexaOp::Store,
            dest: None,
            args: vec![result_reg, arm_val],
            ty: HexaType::Void,
            label: None,
        });

        // Jump to merge
        arm_block.instrs.push(HexaInstr {
            op: HexaOp::Jump,
            dest: None,
            args: vec![merge_id],
            ty: HexaType::Void,
            label: None,
        });
        arm_block.successors.push(merge_id);

        ctx.pending_blocks.push(arm_block);
    }

    // Swap current block to merge block
    let prev_blk = HexaBlock {
        id: block.id,
        instrs: std::mem::take(&mut block.instrs),
        successors: std::mem::take(&mut block.successors),
    };
    ctx.pending_blocks.push(prev_blk);
    block.id = merge_id;

    // Load the result from the result slot
    let loaded = ctx.fresh_reg();
    block.instrs.push(HexaInstr {
        op: HexaOp::Load,
        dest: Some(loaded),
        args: vec![result_reg],
        ty: HexaType::I64,
        label: Some("match_result".to_string()),
    });

    loaded
}

/// Extract a comparable tag from a pattern (for match dispatch).
/// Returns None for catch-all patterns (wildcard, binding, struct-destructure, tuple, guard-only).
fn extract_pattern_tag(ctx: &LowerContext, pattern: &crate::parser::ast::Pattern) -> Option<usize> {
    use crate::parser::ast::Pattern;
    match pattern {
        Pattern::Variant { enum_name, variant_name, .. } => {
            let qualified = format!("{}::{}", enum_name, variant_name);
            Some(ctx.lookup_enum_variant(&qualified).map(|(t, _)| t).unwrap_or(0))
        }
        Pattern::Literal(v, _) => Some(*v as usize),
        Pattern::Guard { pattern: inner, .. } => extract_pattern_tag(ctx, inner),
        Pattern::Wildcard(_) | Pattern::Binding(_, _) |
        Pattern::StructDestructure { .. } | Pattern::Tuple { .. } => None,
    }
}

/// Emit variable bindings for a pattern into the arm block.
fn emit_pattern_bindings(
    ctx: &mut LowerContext,
    block: &mut HexaBlock,
    pattern: &crate::parser::ast::Pattern,
    scrutinee_reg: usize,
) {
    use crate::parser::ast::Pattern;
    match pattern {
        Pattern::Binding(name, _) => {
            ctx.bind_var(name, scrutinee_reg);
        }
        Pattern::Variant { bindings, .. } => {
            for (i, sub_pat) in bindings.iter().enumerate() {
                let field_reg = ctx.fresh_reg();
                block.instrs.push(HexaInstr {
                    op: HexaOp::Load,
                    dest: Some(field_reg),
                    args: vec![scrutinee_reg, i],
                    ty: HexaType::Any,
                    label: Some(format!("variant_field_{}", i)),
                });
                emit_pattern_bindings(ctx, block, sub_pat, field_reg);
            }
        }
        Pattern::StructDestructure { struct_name, fields, .. } => {
            for (field_name, sub_pat) in fields {
                let field_reg = if let Some((_idx, byte_offset, field_ty)) = ctx.lookup_struct_field(struct_name, field_name) {
                    if byte_offset == 0 {
                        let dest = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Load,
                            dest: Some(dest),
                            args: vec![scrutinee_reg],
                            ty: field_ty,
                            label: Some(format!("destr_{}", field_name)),
                        });
                        dest
                    } else {
                        let offset_reg = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Alloc,
                            dest: Some(offset_reg),
                            args: vec![byte_offset],
                            ty: HexaType::I64,
                            label: None,
                        });
                        let addr = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Add,
                            dest: Some(addr),
                            args: vec![scrutinee_reg, offset_reg],
                            ty: HexaType::I64,
                            label: None,
                        });
                        let dest = ctx.fresh_reg();
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Load,
                            dest: Some(dest),
                            args: vec![addr],
                            ty: field_ty,
                            label: Some(format!("destr_{}", field_name)),
                        });
                        dest
                    }
                } else {
                    let dest = ctx.fresh_reg();
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Load,
                        dest: Some(dest),
                        args: vec![scrutinee_reg],
                        ty: HexaType::Any,
                        label: Some(format!("destr_{}", field_name)),
                    });
                    dest
                };
                if let Some(pat) = sub_pat {
                    emit_pattern_bindings(ctx, block, pat, field_reg);
                } else {
                    ctx.bind_var(field_name, field_reg);
                }
            }
        }
        Pattern::Tuple { elements, .. } => {
            for (i, elem_pat) in elements.iter().enumerate() {
                let elem_reg = ctx.fresh_reg();
                block.instrs.push(HexaInstr {
                    op: HexaOp::Load,
                    dest: Some(elem_reg),
                    args: vec![scrutinee_reg, i],
                    ty: HexaType::Any,
                    label: Some(format!("tuple_{}", i)),
                });
                emit_pattern_bindings(ctx, block, elem_pat, elem_reg);
            }
        }
        Pattern::Guard { pattern: inner, .. } => {
            emit_pattern_bindings(ctx, block, inner, scrutinee_reg);
        }
        Pattern::Wildcard(_) | Pattern::Literal(_, _) => {}
    }
}

/// Lower a try expression: `expr?` -> match tag, early return on Err
fn lower_try_expr(
    ctx: &mut LowerContext,
    block: &mut HexaBlock,
    expr: &Expr,
) -> usize {
    let result_reg = lower_expr(ctx, block, expr);

    let ok_id = ctx.fresh_block();
    let err_id = ctx.fresh_block();
    let merge_id = ctx.fresh_block();

    // Result slot for the unwrapped Ok value
    let unwrap_slot = ctx.fresh_reg();
    block.instrs.push(HexaInstr {
        op: HexaOp::Alloc,
        dest: Some(unwrap_slot),
        args: vec![0],
        ty: HexaType::Any,
        label: Some("try_result".to_string()),
    });

    // Load tag (first word: 0=Ok, 1=Err)
    let tag_reg = ctx.fresh_reg();
    block.instrs.push(HexaInstr {
        op: HexaOp::Load,
        dest: Some(tag_reg),
        args: vec![result_reg],
        ty: HexaType::I64,
        label: Some("result_tag".to_string()),
    });

    let zero_reg = ctx.fresh_reg();
    block.instrs.push(HexaInstr {
        op: HexaOp::Alloc,
        dest: Some(zero_reg),
        args: vec![0],
        ty: HexaType::I64,
        label: None,
    });

    let cmp_reg = ctx.fresh_reg();
    block.instrs.push(HexaInstr {
        op: HexaOp::Sub,
        dest: Some(cmp_reg),
        args: vec![tag_reg, zero_reg],
        ty: HexaType::Bool,
        label: Some("eq".to_string()),
    });

    // Branch: tag==0 -> ok, else -> err
    block.instrs.push(HexaInstr {
        op: HexaOp::Branch,
        dest: None,
        args: vec![cmp_reg, err_id, ok_id],
        ty: HexaType::Void,
        label: None,
    });
    block.successors.push(ok_id);
    block.successors.push(err_id);

    // Ok block: unwrap payload, store, jump to merge
    let mut ok_block = HexaBlock { id: ok_id, instrs: Vec::new(), successors: Vec::new() };
    let ok_payload = ctx.fresh_reg();
    ok_block.instrs.push(HexaInstr {
        op: HexaOp::Load,
        dest: Some(ok_payload),
        args: vec![result_reg, 1],
        ty: HexaType::Any,
        label: Some("ok_payload".to_string()),
    });
    ok_block.instrs.push(HexaInstr {
        op: HexaOp::Store,
        dest: None,
        args: vec![unwrap_slot, ok_payload],
        ty: HexaType::Void,
        label: None,
    });
    ok_block.instrs.push(HexaInstr {
        op: HexaOp::Jump,
        dest: None,
        args: vec![merge_id],
        ty: HexaType::Void,
        label: None,
    });
    ok_block.successors.push(merge_id);
    ctx.pending_blocks.push(ok_block);

    // Err block: early return, propagate error
    let mut err_block = HexaBlock { id: err_id, instrs: Vec::new(), successors: Vec::new() };
    err_block.instrs.push(HexaInstr {
        op: HexaOp::Return,
        dest: None,
        args: vec![result_reg],
        ty: HexaType::Void,
        label: Some("try_err_propagate".to_string()),
    });
    ctx.pending_blocks.push(err_block);

    // Swap to merge block
    let prev_blk = HexaBlock {
        id: block.id,
        instrs: std::mem::take(&mut block.instrs),
        successors: std::mem::take(&mut block.successors),
    };
    ctx.pending_blocks.push(prev_blk);
    block.id = merge_id;

    // Load unwrapped value
    let loaded = ctx.fresh_reg();
    block.instrs.push(HexaInstr {
        op: HexaOp::Load,
        dest: Some(loaded),
        args: vec![unwrap_slot],
        ty: HexaType::Any,
        label: Some("try_result".to_string()),
    });

    loaded
}

/// Encode comparison kind in label field for codegen (Sub+Bool instructions)
fn cmp_label(op: &BinOp) -> Option<String> {
    match op {
        BinOp::Eq  => Some("eq".to_string()),
        BinOp::Neq => Some("ne".to_string()),
        BinOp::Lt  => Some("lt".to_string()),
        BinOp::Gt  => Some("gt".to_string()),
        BinOp::Le  => Some("le".to_string()),
        BinOp::Ge  => Some("ge".to_string()),
        _ => None,
    }
}

/// Map AST binary operator to HexaOp + result type
fn lower_binop(op: &BinOp) -> (HexaOp, HexaType) {
    match op {
        BinOp::Add => (HexaOp::Add, HexaType::I64),
        BinOp::Sub => (HexaOp::Sub, HexaType::I64),
        BinOp::Mul => (HexaOp::Mul, HexaType::I64),
        BinOp::Div => (HexaOp::Div, HexaType::I64),
        BinOp::Mod => (HexaOp::Mod, HexaType::I64),
        BinOp::Pow => (HexaOp::Mul, HexaType::I64), // Mk.I: pow approximated as mul
        // Comparison ops → produce Bool, use Sub as underlying op
        BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt |
        BinOp::Le | BinOp::Ge => (HexaOp::Sub, HexaType::Bool),
        // Logic/bitwise → integer arithmetic
        BinOp::And | BinOp::BitAnd => (HexaOp::Mul, HexaType::Bool),
        BinOp::Or | BinOp::BitOr => (HexaOp::Add, HexaType::Bool),
        BinOp::BitXor => (HexaOp::Sub, HexaType::I64),
        BinOp::Range | BinOp::RangeInclusive => (HexaOp::Sub, HexaType::I64), // Mk.I: range as difference
    }
}
