/// HEXA-LANG Parser — n=6 files, Pratt parsing, tau=4 declaration kinds
///
/// Transforms token stream into AST.
/// File count = n = 6: mod.rs, ast.rs, expr.rs, stmt.rs, decl.rs, error.rs

pub mod ast;
pub mod expr;
pub mod stmt;
pub mod decl;
pub mod error;

pub use ast::*;
pub use error::ParseError;

use crate::lexer::Token;
use error::Parser;
use decl::parse_decl;

/// Parse a token stream into a Program AST.
/// Returns Ok(Program) on success, Err(errors) if any parse errors occurred.
pub fn parse(tokens: Vec<Token>) -> Result<Program, Vec<ParseError>> {
    let mut parser = Parser::new(tokens);
    let mut decls = Vec::new();

    while !parser.is_eof() {
        match parse_decl(&mut parser) {
            Ok(d) => decls.push(d),
            Err(e) => parser.error_and_sync(e),
        }
    }

    if parser.errors.is_empty() {
        Ok(Program { decls })
    } else {
        Err(parser.errors)
    }
}
