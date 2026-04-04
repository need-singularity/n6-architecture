/// Name resolution — scope-stack based symbol lookup
///
/// Maintains a stack of lexical scopes, each mapping names to VarInfo.
/// Used by both the type checker and the ownership analyzer.

use std::collections::HashMap;
use crate::lexer::Span;
use super::error::SemaError;

/// Information stored about each variable binding
#[derive(Clone, Debug)]
pub struct VarInfo {
    pub name: String,
    pub ty_name: String,    // resolved type name (e.g., "i64", "MyStruct")
    pub mutable: bool,
    pub defined_at: Span,
}

/// A single lexical scope
#[derive(Clone, Debug)]
pub struct Scope {
    pub bindings: HashMap<String, VarInfo>,
}

impl Scope {
    pub fn new() -> Self {
        Scope { bindings: HashMap::new() }
    }
}

/// Scope-stack resolver for name lookup
pub struct Resolver {
    scopes: Vec<Scope>,
}

impl Resolver {
    pub fn new() -> Self {
        // Start with a global scope
        Resolver { scopes: vec![Scope::new()] }
    }

    /// Push a new nested scope (entering a block/function)
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Pop the innermost scope (leaving a block/function)
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Define a new binding in the current (innermost) scope.
    /// Returns error if the name is already defined in the *same* scope.
    pub fn define(&mut self, info: VarInfo) -> Result<(), SemaError> {
        let scope = self.scopes.last_mut().expect("scope stack empty");
        if scope.bindings.contains_key(&info.name) {
            return Err(SemaError::DuplicateDefinition {
                span: info.defined_at,
                name: info.name.clone(),
            });
        }
        scope.bindings.insert(info.name.clone(), info);
        Ok(())
    }

    /// Look up a name, searching from innermost to outermost scope.
    /// Returns the VarInfo if found, or UndefinedName error.
    pub fn lookup(&self, name: &str, at: Span) -> Result<&VarInfo, SemaError> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.bindings.get(name) {
                return Ok(info);
            }
        }
        Err(SemaError::UndefinedName {
            span: at,
            name: name.to_string(),
        })
    }

    /// Check if a name is defined anywhere in the scope stack
    pub fn is_defined(&self, name: &str) -> bool {
        self.scopes.iter().rev().any(|s| s.bindings.contains_key(name))
    }

    /// Current nesting depth (0 = global)
    pub fn depth(&self) -> usize {
        self.scopes.len() - 1
    }
}
