/// Trait resolution — Mk.II: trait + impl verification
///
/// Validates trait implementations and builds dispatch tables.

use std::collections::HashMap;
use crate::parser::ast::{Program, Decl};
use super::error::SemaError;

/// Trait resolver state
pub struct TraitResolver {
    trait_methods: HashMap<String, Vec<String>>,
}

impl TraitResolver {
    pub fn new() -> Self {
        TraitResolver { trait_methods: HashMap::new() }
    }

    /// Resolve all trait implementations in the program.
    pub fn resolve_traits(&mut self, program: &Program) -> Result<(), Vec<SemaError>> {
        let mut errors = Vec::new();

        // Collect trait definitions
        for decl in &program.decls {
            if let Decl::TraitDecl(td) = decl {
                let methods: Vec<String> = td.methods.iter().map(|m| m.name.clone()).collect();
                self.trait_methods.insert(td.name.clone(), methods);
            }
        }

        // Verify impl blocks
        for decl in &program.decls {
            if let Decl::ImplBlock(ib) = decl {
                let required = match self.trait_methods.get(&ib.trait_name) {
                    Some(m) => m.clone(),
                    None => {
                        errors.push(SemaError::UndefinedName {
                            span: ib.span,
                            name: format!("trait '{}'", ib.trait_name),
                        });
                        continue;
                    }
                };
                let impl_names: Vec<&str> = ib.methods.iter().map(|m| m.name.as_str()).collect();
                for req in &required {
                    if !impl_names.contains(&req.as_str()) {
                        errors.push(SemaError::TypeError {
                            span: ib.span,
                            expected: format!("method '{}' from trait '{}'", req, ib.trait_name),
                            found: "missing".to_string(),
                        });
                    }
                }
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
