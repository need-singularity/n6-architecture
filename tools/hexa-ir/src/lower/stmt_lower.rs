/// Statement lowering — AST Stmt -> HEXA-IR instructions + blocks
///
/// Let   -> Alloc + Store
/// If    -> Branch + Phi across then/else blocks
/// While -> Loop header + body + back-edge
/// Return -> Return op

use crate::ir::*;
use crate::parser::ast::{Stmt, Block as AstBlock};
use super::LowerContext;
use super::expr_lower::lower_expr;

/// Lower a block of statements into the current HexaBlock,
/// creating new blocks as needed (for control flow) and appending them to `extra_blocks`.
pub fn lower_block(
    ctx: &mut LowerContext,
    block: &mut HexaBlock,
    ast_block: &AstBlock,
    extra_blocks: &mut Vec<HexaBlock>,
) {
    for stmt in &ast_block.stmts {
        lower_stmt_full(ctx, block, stmt, extra_blocks);
    }
}

/// Lower a single statement, returning the "result" register (0 for void stmts).
/// This simplified version does not create extra blocks (used by expr_lower for Block exprs).
pub fn lower_stmt(ctx: &mut LowerContext, block: &mut HexaBlock, stmt: &Stmt) -> usize {
    let mut extra = Vec::new();
    lower_stmt_full(ctx, block, stmt, &mut extra);
    // In the simplified path, extra blocks are appended inline
    // (the caller in expr_lower doesn't handle multi-block; Mk.II will fix this)
    ctx.reg_counter.saturating_sub(1)
}

/// Full statement lowering with extra block creation
fn lower_stmt_full(
    ctx: &mut LowerContext,
    block: &mut HexaBlock,
    stmt: &Stmt,
    extra_blocks: &mut Vec<HexaBlock>,
) {
    match stmt {
        Stmt::Let { name, init, ty, .. } => {
            // Allocate space for the variable
            let addr = ctx.fresh_reg();
            let var_ty = ty.as_ref()
                .map(|t| super::lower_type_expr(t))
                .unwrap_or(HexaType::Any);
            block.instrs.push(HexaInstr {
                op: HexaOp::Alloc,
                dest: Some(addr),
                args: vec![],
                ty: var_ty.clone(),
            label: None,
            });

            // If there's an initializer, lower it and store
            if let Some(init_expr) = init {
                let val = lower_expr(ctx, block, init_expr);
                block.instrs.push(HexaInstr {
                    op: HexaOp::Store,
                    dest: None,
                    args: vec![addr, val],
                    ty: HexaType::Void,
            label: None,
                });
                // Emit ownership transfer: the value is now owned by this variable
                super::proof_emit::emit_ownership_transfer(ctx, block, val, addr);
            }

            ctx.bind_var(name, addr);
        }

        Stmt::Assign { target, value, .. } => {
            let val = lower_expr(ctx, block, value);
            match target {
                crate::parser::ast::Expr::Ident(name, _) => {
                    if let Some(addr) = ctx.lookup_var(name) {
                        block.instrs.push(HexaInstr {
                            op: HexaOp::Store,
                            dest: None,
                            args: vec![addr, val],
                            ty: HexaType::Void,
            label: None,
                        });
                    }
                }
                _ => {
                    // Complex lvalue (field, index) — lower target as address
                    let addr = lower_expr(ctx, block, target);
                    block.instrs.push(HexaInstr {
                        op: HexaOp::Store,
                        dest: None,
                        args: vec![addr, val],
                        ty: HexaType::Void,
            label: None,
                    });
                }
            }
        }

        Stmt::Return { value, .. } => {
            if let Some(expr) = value {
                let val = lower_expr(ctx, block, expr);
                block.instrs.push(HexaInstr {
                    op: HexaOp::Return,
                    dest: None,
                    args: vec![val],
                    ty: HexaType::Void,
            label: None,
                });
            } else {
                block.instrs.push(HexaInstr {
                    op: HexaOp::Return,
                    dest: None,
                    args: vec![],
                    ty: HexaType::Void,
            label: None,
                });
            }
        }

        Stmt::If { cond, then_block, else_block, .. } => {
            let cond_reg = lower_expr(ctx, block, cond);

            // Create then-block
            let then_id = ctx.fresh_block();
            let mut then_blk = HexaBlock {
                id: then_id,
                instrs: Vec::new(),
                successors: Vec::new(),
            };

            // Create merge block (joins after if/else)
            let merge_id = ctx.fresh_block();
            let mut merge_blk = HexaBlock {
                id: merge_id,
                instrs: Vec::new(),
                successors: Vec::new(),
            };

            if let Some(else_ast) = else_block {
                // Create else-block
                let else_id = ctx.fresh_block();
                let mut else_blk = HexaBlock {
                    id: else_id,
                    instrs: Vec::new(),
                    successors: Vec::new(),
                };

                // Emit Branch in current block
                block.instrs.push(HexaInstr {
                    op: HexaOp::Branch,
                    dest: None,
                    args: vec![cond_reg, then_id, else_id],
                    ty: HexaType::Void,
            label: None,
                });
                block.successors.push(then_id);
                block.successors.push(else_id);

                // Lower then body
                lower_block(ctx, &mut then_blk, then_block, extra_blocks);
                then_blk.instrs.push(HexaInstr {
                    op: HexaOp::Jump,
                    dest: None,
                    args: vec![merge_id],
                    ty: HexaType::Void,
            label: None,
                });
                then_blk.successors.push(merge_id);

                // Lower else body
                lower_block(ctx, &mut else_blk, else_ast, extra_blocks);
                else_blk.instrs.push(HexaInstr {
                    op: HexaOp::Jump,
                    dest: None,
                    args: vec![merge_id],
                    ty: HexaType::Void,
            label: None,
                });
                else_blk.successors.push(merge_id);

                extra_blocks.push(then_blk);
                extra_blocks.push(else_blk);
            } else {
                // No else: branch to then or merge
                block.instrs.push(HexaInstr {
                    op: HexaOp::Branch,
                    dest: None,
                    args: vec![cond_reg, then_id, merge_id],
                    ty: HexaType::Void,
            label: None,
                });
                block.successors.push(then_id);
                block.successors.push(merge_id);

                // Lower then body
                lower_block(ctx, &mut then_blk, then_block, extra_blocks);
                then_blk.instrs.push(HexaInstr {
                    op: HexaOp::Jump,
                    dest: None,
                    args: vec![merge_id],
                    ty: HexaType::Void,
            label: None,
                });
                then_blk.successors.push(merge_id);

                extra_blocks.push(then_blk);
            }

            // Swap current block with merge block so subsequent statements
            // (after the if/else) are emitted into the merge block
            let prev_blk = HexaBlock {
                id: block.id,
                instrs: std::mem::take(&mut block.instrs),
                successors: std::mem::take(&mut block.successors),
            };
            block.id = merge_id;
            extra_blocks.push(prev_blk);
            // merge_blk is now `block` (subsequent code goes here)
        }

        Stmt::While { cond, body, .. } => {
            // Create loop header block
            let header_id = ctx.fresh_block();
            let mut header_blk = HexaBlock {
                id: header_id,
                instrs: Vec::new(),
                successors: Vec::new(),
            };

            // Create loop body block
            let body_id = ctx.fresh_block();
            let mut body_blk = HexaBlock {
                id: body_id,
                instrs: Vec::new(),
                successors: Vec::new(),
            };

            // Create exit block
            let exit_id = ctx.fresh_block();

            // Current block jumps to header
            block.instrs.push(HexaInstr {
                op: HexaOp::Jump,
                dest: None,
                args: vec![header_id],
                ty: HexaType::Void,
            label: None,
            });
            block.successors.push(header_id);

            // Save the current block (with Jump) and replace it with the exit block.
            // This way, any subsequent statements after the while loop are emitted
            // into the exit block (which is now `block`).
            let prev_blk = HexaBlock {
                id: block.id,
                instrs: std::mem::take(&mut block.instrs),
                successors: std::mem::take(&mut block.successors),
            };
            block.id = exit_id;
            // Push the original block content as a separate block
            extra_blocks.push(prev_blk);

            // Header: evaluate condition, branch to body or exit
            let cond_reg = lower_expr(ctx, &mut header_blk, cond);
            header_blk.instrs.push(HexaInstr {
                op: HexaOp::Branch,
                dest: None,
                args: vec![cond_reg, body_id, exit_id],
                ty: HexaType::Void,
            label: None,
            });
            header_blk.successors.push(body_id);
            header_blk.successors.push(exit_id);

            // Body: lower statements, then jump back to header (back-edge)
            lower_block(ctx, &mut body_blk, body, extra_blocks);
            body_blk.instrs.push(HexaInstr {
                op: HexaOp::Jump,
                dest: None,
                args: vec![header_id],
                ty: HexaType::Void,
            label: None,
            });
            body_blk.successors.push(header_id);

            extra_blocks.push(header_blk);
            extra_blocks.push(body_blk);
            // Note: the exit block is now `block` itself — subsequent statements
            // will be appended to it, so we don't push it to extra_blocks.
        }

        Stmt::ExprStmt { expr, .. } => {
            // Lower the expression; discard result
            let _reg = lower_expr(ctx, block, expr);
        }
    }
}
