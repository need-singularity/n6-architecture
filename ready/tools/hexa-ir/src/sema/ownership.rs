/// Ownership analysis — Layer 2 (single-owner, move + borrow checking)
///
/// Tracks the ownership state of each variable through the AST.
/// Enforces:
///   - Move semantics: `let y = x;` moves x (non-Copy types)
///   - Use-after-move detection
///   - Immutable borrow: `let r = &x;` prevents mutation of x
///   - Mutable borrow exclusivity: `let r = &mut x;` prevents other borrows
///   - Function argument ownership transfer

use std::collections::HashMap;
use crate::lexer::Span;
use crate::parser::ast::*;
use super::error::SemaError;

/// Whether a variable's type is Copy (primitives) or requires move semantics
fn is_copy_type_name(name: &str) -> bool {
    matches!(name, "i64" | "f64" | "bool" | "char" | "byte" | "void" | "any")
}

/// Ownership state of a variable
#[derive(Clone, Debug, PartialEq)]
pub enum OwnershipState {
    /// Variable is live and owned
    Owned,
    /// Variable is immutably borrowed (count of active borrows)
    ImmutBorrowed { count: usize },
    /// Variable is mutably borrowed (exclusive)
    MutBorrowed { borrower: String, at: Span },
    /// Variable has been moved — any further use is an error
    Moved { moved_at: Span },
}

/// Per-variable tracking entry
#[derive(Clone, Debug)]
struct VarState {
    name: String,
    state: OwnershipState,
    mutable: bool,
    is_copy: bool,
    defined_at: Span,
}

/// Ownership checker: walks statements and tracks variable states
pub struct OwnershipChecker {
    /// Variable name -> current state
    states: HashMap<String, VarState>,
    errors: Vec<SemaError>,
}

impl OwnershipChecker {
    pub fn new() -> Self {
        OwnershipChecker {
            states: HashMap::new(),
            errors: Vec::new(),
        }
    }

    /// Register a new variable as Owned
    pub fn define(&mut self, name: &str, span: Span) {
        self.define_with_info(name, span, false, true);
    }

    /// Register a variable with mutability and copy info
    pub fn define_with_info(&mut self, name: &str, span: Span, mutable: bool, is_copy: bool) {
        self.states.insert(name.to_string(), VarState {
            name: name.to_string(),
            state: OwnershipState::Owned,
            mutable,
            is_copy,
            defined_at: span,
        });
    }

    /// Record a move of a variable (e.g., `let y = x;` for non-Copy types)
    pub fn check_move(&mut self, name: &str, at: Span) -> Result<(), SemaError> {
        match self.states.get(name) {
            Some(vs) => {
                // Copy types don't move
                if vs.is_copy {
                    return Ok(());
                }
                match &vs.state {
                    OwnershipState::Moved { moved_at } => {
                        Err(SemaError::OwnershipError {
                            span: at,
                            message: format!(
                                "use of moved value '{}' (moved at {})",
                                name, moved_at
                            ),
                        })
                    }
                    OwnershipState::MutBorrowed { borrower, at: borrow_span } => {
                        Err(SemaError::OwnershipError {
                            span: at,
                            message: format!(
                                "cannot move '{}' while mutably borrowed by '{}' (borrowed at {})",
                                name, borrower, borrow_span
                            ),
                        })
                    }
                    OwnershipState::ImmutBorrowed { count } if *count > 0 => {
                        Err(SemaError::OwnershipError {
                            span: at,
                            message: format!(
                                "cannot move '{}' while it has {} active immutable borrow(s)",
                                name, count
                            ),
                        })
                    }
                    _ => {
                        // Mark as moved
                        if let Some(vs) = self.states.get_mut(name) {
                            vs.state = OwnershipState::Moved { moved_at: at };
                        }
                        Ok(())
                    }
                }
            }
            None => Ok(()),
        }
    }

    /// Record an immutable borrow of a variable (`&x`)
    pub fn check_borrow(&mut self, name: &str, at: Span) -> Result<(), SemaError> {
        match self.states.get(name) {
            Some(vs) => {
                match &vs.state {
                    OwnershipState::Moved { moved_at } => {
                        Err(SemaError::OwnershipError {
                            span: at,
                            message: format!(
                                "cannot borrow moved value '{}' (moved at {})",
                                name, moved_at
                            ),
                        })
                    }
                    OwnershipState::MutBorrowed { borrower, at: borrow_span } => {
                        Err(SemaError::OwnershipError {
                            span: at,
                            message: format!(
                                "cannot immutably borrow '{}' while mutably borrowed by '{}' (at {})",
                                name, borrower, borrow_span
                            ),
                        })
                    }
                    OwnershipState::ImmutBorrowed { count } => {
                        // Allow multiple immutable borrows
                        let new_count = count + 1;
                        if let Some(vs) = self.states.get_mut(name) {
                            vs.state = OwnershipState::ImmutBorrowed { count: new_count };
                        }
                        Ok(())
                    }
                    _ => {
                        if let Some(vs) = self.states.get_mut(name) {
                            vs.state = OwnershipState::ImmutBorrowed { count: 1 };
                        }
                        Ok(())
                    }
                }
            }
            None => Ok(()),
        }
    }

    /// Record a mutable borrow of a variable (`&mut x`)
    pub fn check_mut_borrow(&mut self, name: &str, borrower: &str, at: Span) -> Result<(), SemaError> {
        match self.states.get(name) {
            Some(vs) => {
                if !vs.mutable {
                    return Err(SemaError::OwnershipError {
                        span: at,
                        message: format!(
                            "cannot mutably borrow immutable variable '{}'",
                            name
                        ),
                    });
                }
                match &vs.state {
                    OwnershipState::Moved { moved_at } => {
                        Err(SemaError::OwnershipError {
                            span: at,
                            message: format!(
                                "cannot borrow moved value '{}' (moved at {})",
                                name, moved_at
                            ),
                        })
                    }
                    OwnershipState::MutBorrowed { borrower: existing, at: borrow_span } => {
                        Err(SemaError::OwnershipError {
                            span: at,
                            message: format!(
                                "cannot mutably borrow '{}': already mutably borrowed by '{}' (at {})",
                                name, existing, borrow_span
                            ),
                        })
                    }
                    OwnershipState::ImmutBorrowed { count } if *count > 0 => {
                        Err(SemaError::OwnershipError {
                            span: at,
                            message: format!(
                                "cannot mutably borrow '{}' while immutably borrowed ({} active borrow(s))",
                                name, count
                            ),
                        })
                    }
                    _ => {
                        if let Some(vs) = self.states.get_mut(name) {
                            vs.state = OwnershipState::MutBorrowed {
                                borrower: borrower.to_string(),
                                at,
                            };
                        }
                        Ok(())
                    }
                }
            }
            None => Ok(()),
        }
    }

    /// Check mutation of a variable (assignment to it)
    pub fn check_mutation(&self, name: &str, at: Span) -> Result<(), SemaError> {
        if let Some(vs) = self.states.get(name) {
            match &vs.state {
                OwnershipState::ImmutBorrowed { count } if *count > 0 => {
                    return Err(SemaError::OwnershipError {
                        span: at,
                        message: format!(
                            "cannot assign to '{}' while immutably borrowed ({} active borrow(s))",
                            name, count
                        ),
                    });
                }
                OwnershipState::Moved { moved_at } => {
                    return Err(SemaError::OwnershipError {
                        span: at,
                        message: format!(
                            "cannot assign to moved value '{}' (moved at {})",
                            name, moved_at
                        ),
                    });
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Check a use of a variable (read): fails if moved
    pub fn check_use(&self, name: &str, at: Span) -> Result<(), SemaError> {
        if let Some(vs) = self.states.get(name) {
            if let OwnershipState::Moved { moved_at } = &vs.state {
                return Err(SemaError::OwnershipError {
                    span: at,
                    message: format!(
                        "use of moved value '{}' (moved at {})",
                        name, moved_at
                    ),
                });
            }
        }
        Ok(())
    }

    /// Walk a block of statements, tracking ownership
    pub fn check_block(&mut self, block: &Block) -> Vec<SemaError> {
        let mut errors = Vec::new();
        for stmt in &block.stmts {
            if let Err(e) = self.check_stmt(stmt) {
                errors.push(e);
            }
        }
        errors
    }

    /// Check a single statement for ownership violations
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), SemaError> {
        match stmt {
            Stmt::Let { name, init, span, mutable, ty, .. } => {
                // Determine if the new variable's type is Copy
                let is_copy = ty.as_ref()
                    .map(|te| is_copy_type_expr(te))
                    .unwrap_or_else(|| {
                        // Infer from initializer
                        match init {
                            Some(Expr::IntLit(_, _)) | Some(Expr::FloatLit(_, _)) |
                            Some(Expr::BoolLit(_, _)) => true,
                            Some(Expr::Ident(src_name, _)) => {
                                self.states.get(src_name.as_str())
                                    .map(|vs| vs.is_copy)
                                    .unwrap_or(true)
                            }
                            _ => false, // structs, arrays, etc. are not Copy
                        }
                    });

                if let Some(expr) = init {
                    // Check for move semantics: `let y = x;` moves x
                    self.check_expr_ownership(expr, *span)?;
                }
                self.define_with_info(name, *span, *mutable, is_copy);
                Ok(())
            }
            Stmt::Assign { target, value, span } => {
                // Check the value expression for ownership
                self.check_expr_ownership(value, *span)?;
                // Check mutation is allowed
                if let Expr::Ident(name, id_span) = target {
                    self.check_mutation(name, *id_span)?;
                    // Re-assignment restores ownership
                    self.define(name, *span);
                }
                Ok(())
            }
            Stmt::Return { value, .. } => {
                if let Some(expr) = value {
                    self.check_expr_uses(expr)?;
                }
                Ok(())
            }
            Stmt::If { cond, then_block, else_block, .. } => {
                self.check_expr_uses(cond)?;
                let errs = self.check_block(then_block);
                if let Some(first) = errs.into_iter().next() {
                    return Err(first);
                }
                if let Some(eb) = else_block {
                    let errs = self.check_block(eb);
                    if let Some(first) = errs.into_iter().next() {
                        return Err(first);
                    }
                }
                Ok(())
            }
            Stmt::While { cond, body, .. } => {
                self.check_expr_uses(cond)?;
                let errs = self.check_block(body);
                if let Some(first) = errs.into_iter().next() {
                    return Err(first);
                }
                Ok(())
            }
            Stmt::ForLoop { var, iterable, body, span, .. } => {
                self.check_expr_uses(iterable)?;
                self.define(var, *span);
                let errs = self.check_block(body);
                if let Some(first) = errs.into_iter().next() {
                    return Err(first);
                }
                Ok(())
            }
            Stmt::ExprStmt { expr, .. } => {
                self.check_expr_uses(expr)
            }
        }
    }

    /// Check ownership transfer in expressions (handles move on init).
    /// For `let y = x;`, if x is non-Copy, this moves x.
    fn check_expr_ownership(&mut self, expr: &Expr, at: Span) -> Result<(), SemaError> {
        match expr {
            Expr::Ident(name, span) => {
                // Moving a variable via assignment
                self.check_use(name, *span)?;
                self.check_move(name, *span)?;
                Ok(())
            }
            Expr::Call { func, args, .. } => {
                // Function arguments transfer ownership for non-Copy types
                self.check_expr_uses(func)?;
                for arg in args {
                    match arg {
                        Expr::Ident(name, span) => {
                            self.check_use(name, *span)?;
                            self.check_move(name, *span)?;
                        }
                        _ => {
                            self.check_expr_uses(arg)?;
                        }
                    }
                }
                Ok(())
            }
            _ => self.check_expr_uses(expr),
        }
    }

    /// Check all variable uses in an expression (read-only check)
    fn check_expr_uses(&self, expr: &Expr) -> Result<(), SemaError> {
        match expr {
            Expr::Ident(name, span) => self.check_use(name, *span),
            Expr::Binary { lhs, rhs, .. } => {
                self.check_expr_uses(lhs)?;
                self.check_expr_uses(rhs)
            }
            Expr::Unary { operand, .. } => self.check_expr_uses(operand),
            Expr::Call { func, args, .. } => {
                self.check_expr_uses(func)?;
                for arg in args {
                    self.check_expr_uses(arg)?;
                }
                Ok(())
            }
            Expr::Field { obj, .. } => self.check_expr_uses(obj),
            Expr::Index { arr, idx, .. } => {
                self.check_expr_uses(arr)?;
                self.check_expr_uses(idx)
            }
            Expr::StructInit { fields, .. } => {
                for (_, val) in fields {
                    self.check_expr_uses(val)?;
                }
                Ok(())
            }
            Expr::Match { scrutinee, arms, .. } => {
                self.check_expr_uses(scrutinee)?;
                for arm in arms {
                    self.check_expr_uses(&arm.body)?;
                }
                Ok(())
            }
            Expr::ArrayLit { elements, .. } => {
                for elem in elements {
                    self.check_expr_uses(elem)?;
                }
                Ok(())
            }
            Expr::Closure { body, .. } => {
                self.check_expr_uses(body)
            }
            // Literals don't reference variables
            _ => Ok(()),
        }
    }

    /// Consume the checker and return all collected errors
    pub fn into_errors(self) -> Vec<SemaError> {
        self.errors
    }
}

/// Check if a TypeExpr represents a Copy type
fn is_copy_type_expr(te: &TypeExpr) -> bool {
    match te {
        TypeExpr::Named(name, _) => is_copy_type_name(name),
        _ => false,
    }
}
