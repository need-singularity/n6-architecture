/// Statement parser — n=6 statement kinds
///
/// Parses: let, assign, return, if, while, expr-stmt

use crate::lexer::{TokenKind, Span};
use super::ast::*;
use super::error::{Parser, ParseError};
use super::expr::parse_expr;

/// Parse a single statement
pub fn parse_stmt(p: &mut Parser) -> Result<Stmt, ParseError> {
    match p.peek() {
        TokenKind::Let => parse_let(p),
        TokenKind::Return => parse_return(p),
        TokenKind::If => parse_if(p),
        TokenKind::While => parse_while(p),
        _ => parse_assign_or_expr(p),
    }
}

/// `let [mut] name [: type] [= init];`
fn parse_let(p: &mut Parser) -> Result<Stmt, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Let)?;

    let mutable = p.eat(&TokenKind::Mut);
    let (name, _) = p.expect_ident()?;

    // Optional type annotation
    let ty = if p.eat(&TokenKind::Colon) {
        Some(parse_type_expr(p)?)
    } else {
        None
    };

    // Optional initializer
    let init = if p.eat(&TokenKind::Assign) {
        Some(parse_expr(p)?)
    } else {
        None
    };

    let end = p.peek_span();
    // Optional semicolon (Go/Kotlin style): accept ; if present, but don't require it
    p.eat(&TokenKind::Semicolon);

    Ok(Stmt::Let {
        name,
        mutable,
        ty,
        init,
        span: start.merge(end),
    })
}

/// `return [expr];`
fn parse_return(p: &mut Parser) -> Result<Stmt, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::Return)?;

    let value = if !p.at(&TokenKind::Semicolon) && !p.at(&TokenKind::RBrace) && !p.is_eof() {
        Some(parse_expr(p)?)
    } else {
        None
    };

    let end = p.peek_span();
    // Optional semicolon (Go/Kotlin style)
    p.eat(&TokenKind::Semicolon);

    Ok(Stmt::Return { value, span: start.merge(end) })
}

/// `if cond { block } [else { block }]`
fn parse_if(p: &mut Parser) -> Result<Stmt, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::If)?;

    let cond = parse_expr(p)?;
    let then_block = parse_block(p)?;

    let else_block = if p.eat(&TokenKind::Else) {
        Some(parse_block(p)?)
    } else {
        None
    };

    let end = else_block.as_ref()
        .map(|b| b.span)
        .unwrap_or(then_block.span);

    Ok(Stmt::If {
        cond,
        then_block,
        else_block,
        span: start.merge(end),
    })
}

/// `while cond { block }`
fn parse_while(p: &mut Parser) -> Result<Stmt, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::While)?;

    let cond = parse_expr(p)?;
    let body = parse_block(p)?;

    Ok(Stmt::While {
        span: start.merge(body.span),
        cond,
        body,
    })
}

/// Expression statement or assignment: `expr;` or `target = value;`
fn parse_assign_or_expr(p: &mut Parser) -> Result<Stmt, ParseError> {
    let start = p.peek_span();
    let expr = parse_expr(p)?;

    if p.eat(&TokenKind::Assign) {
        let value = parse_expr(p)?;
        let end = p.peek_span();
        // Optional semicolon (Go/Kotlin style)
        p.eat(&TokenKind::Semicolon);
        Ok(Stmt::Assign {
            target: expr,
            value,
            span: start.merge(end),
        })
    } else {
        let end = p.peek_span();
        // Optional semicolon (Go/Kotlin style)
        p.eat(&TokenKind::Semicolon);
        Ok(Stmt::ExprStmt {
            expr,
            span: start.merge(end),
        })
    }
}

/// Parse a brace-delimited block: `{ stmt* }`
pub fn parse_block(p: &mut Parser) -> Result<Block, ParseError> {
    let start = p.peek_span();
    p.expect(&TokenKind::LBrace)?;

    let mut stmts = Vec::new();
    while !p.at(&TokenKind::RBrace) && !p.is_eof() {
        match parse_stmt(p) {
            Ok(stmt) => stmts.push(stmt),
            Err(err) => p.error_and_sync(err),
        }
    }

    let end = p.peek_span();
    p.expect(&TokenKind::RBrace)?;

    Ok(Block { stmts, span: start.merge(end) })
}

/// Parse a type expression: `i64`, `[i64; 6]`, `fn(i64) -> bool`
pub fn parse_type_expr(p: &mut Parser) -> Result<TypeExpr, ParseError> {
    let span = p.peek_span();

    match p.peek().clone() {
        // Array type: [T; N]
        TokenKind::LBracket => {
            p.advance();
            let elem = parse_type_expr(p)?;
            p.expect(&TokenKind::Semicolon)?;
            let size_span = p.peek_span();
            match p.peek() {
                TokenKind::IntLit(n) => {
                    let n = *n as usize;
                    p.advance();
                    let end = p.peek_span();
                    p.expect(&TokenKind::RBracket)?;
                    Ok(TypeExpr::Array(Box::new(elem), n, span.merge(end)))
                }
                _ => Err(ParseError::new(size_span, "expected array size")),
            }
        }
        // Function type: fn(T1, T2) -> R
        TokenKind::Fn => {
            p.advance();
            p.expect(&TokenKind::LParen)?;
            let mut params = Vec::new();
            while !p.at(&TokenKind::RParen) && !p.is_eof() {
                params.push(parse_type_expr(p)?);
                if !p.eat(&TokenKind::Comma) {
                    break;
                }
            }
            p.expect(&TokenKind::RParen)?;
            p.expect(&TokenKind::Arrow)?;
            let ret = parse_type_expr(p)?;
            let end = ret.span();
            Ok(TypeExpr::Fn(params, Box::new(ret), span.merge(end)))
        }
        // Named type (identifiers or type keywords)
        TokenKind::Ident(ref name) => {
            let name = name.clone();
            p.advance();
            Ok(TypeExpr::Named(name, span))
        }
        TokenKind::KwI64 => { p.advance(); Ok(TypeExpr::Named("i64".into(), span)) }
        TokenKind::KwF64 => { p.advance(); Ok(TypeExpr::Named("f64".into(), span)) }
        TokenKind::KwBool => { p.advance(); Ok(TypeExpr::Named("bool".into(), span)) }
        TokenKind::KwChar => { p.advance(); Ok(TypeExpr::Named("char".into(), span)) }
        TokenKind::KwStr => { p.advance(); Ok(TypeExpr::Named("str".into(), span)) }
        TokenKind::KwByte => { p.advance(); Ok(TypeExpr::Named("byte".into(), span)) }
        TokenKind::KwVoid => { p.advance(); Ok(TypeExpr::Named("void".into(), span)) }
        TokenKind::KwAny => { p.advance(); Ok(TypeExpr::Named("any".into(), span)) }
        other => Err(ParseError::new(span, format!("expected type, found {:?}", other))),
    }
}
